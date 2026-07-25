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

use crate::model::defaults::Defaults;

use crate::model::traits::{AsyncReceiver, AsyncSender};
use crate::view::view::View;
use axum::Router;

use axum::routing::get;
use tokio::sync::{Mutex, RwLock};

use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let defaults = Defaults::new();

    let eventbus = Arc::new(Eventbus::new());

    let mut collector = Collector::new(eventbus.publish());
    let mut view = View::new(eventbus.subscribe());

    let mut websocket = Arc::new(WebSocketClient::new(eventbus.clone()));
    let current_event_state = Arc::new(RwLock::new(defaults.default_event));

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
        .fallback_service(ServeDir::new(defaults.frontend_path))
        .layer(cors);

    let addr = format!("0.0.0.0:{}", defaults.port.to_string());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
