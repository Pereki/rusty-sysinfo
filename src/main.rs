mod api;
mod collector;
mod eventbus;
mod model;
mod view;

use std::sync::Arc;

use crate::api::rest::RestClient;
use crate::api::websocket::WebSocketClient;
use crate::collector::collector::Collector;
use crate::eventbus::eventbus::Eventbus;
use crate::model::cpu_info::CpuInfo;
use crate::model::event::Event;
use crate::model::memory_info::MemoryInfo;
use crate::model::traits::{AsyncReceiver, AsyncSender};
use crate::view::view::View;
use axum::Router;
use axum::handler::HandlerWithoutStateExt;
use axum::response::Html;
use axum::routing::get;
use tokio::sync::{Mutex, RwLock};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

async fn spa_fallback() -> Html<String> {
    match tokio::fs::read_to_string("frontend/sysinfo-frontend/dist/index.html").await {
        Ok(content) => Html(content),
        Err(_) => Html("<h1>Frontend not built</h1><p>Run pnpm build</p>".into()),
    }
}

#[tokio::main]
async fn main() {
    let eventbus = Arc::new(Eventbus::new());
    let static_dir = format!("{}/frontend-dir", env!("CARGO_MANIFEST_DIR"));

    let mut collector = Collector::new(eventbus.publish());
    let mut view = View::new(eventbus.subscribe());

    let mut websocket = Arc::new(WebSocketClient::new(eventbus.clone()));
    let current_event_state = Arc::new(RwLock::new(Event::new(
        crate::model::event::EventType::UPDATE,
        MemoryInfo::new(0, 0),
        CpuInfo::new(0.00),
    )));

    let rest_client = Arc::new(Mutex::new(RestClient::new(current_event_state.clone())));

    let collector_task = tokio::spawn(async move { collector.run().await });

    let rest_client_clone = rest_client.clone();
    let mut rest_receiver = eventbus.subscribe();
    let rest_client_task = tokio::spawn(async move {
        let mut guard = rest_client_clone.lock().await;
        guard.receive(&mut rest_receiver).await;
    });

    let mut view_receiver = eventbus.subscribe();
    let view_task = tokio::spawn(async move { view.receive(&mut view_receiver).await });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/status", get(RestClient::get_latest_event))
        .with_state(rest_client.clone())
        .route("/ws", get(WebSocketClient::setup_ws))
        .with_state(websocket.clone())
        .fallback_service(ServeDir::new(static_dir).fallback(spa_fallback.into_service()))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

pub async fn start_async_task() {}
