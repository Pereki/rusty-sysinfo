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
use crate::view::view::View;
use axum::Router;
use axum::extract::State;
use axum::handler::{Handler, HandlerWithoutStateExt};
use axum::response::Html;
use axum::routing::get;
use tokio::sync::RwLock;
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

    let mut rest_client = Arc::new(RestClient::new(current_event_state.clone()));

    let collector_task = tokio::spawn(async move { collector.start().await });

    let mut cloned_client = rest_client.clone();
    let rest_client_task =
        tokio::spawn(async move { cloned_client.receive(&mut eventbus.subscribe()).await });
    let view_task = tokio::spawn(async move { view.listen().await });

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

    let _ = tokio::join!(collector_task, view_task, rest_client_task);
}
