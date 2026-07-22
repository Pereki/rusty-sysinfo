mod api;
mod collector;
mod eventbus;
mod model;
mod view;

use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::routing::get;
use tokio::sync::RwLock;

use crate::api::rest::RestClient;
use crate::collector::collector::Collector;
use crate::eventbus::eventbus::Eventbus;
use crate::model::cpu_info::CpuInfo;
use crate::model::event::Event;
use crate::model::memory_info::MemoryInfo;
use crate::view::view::View;

#[tokio::main]
async fn main() {
    let eventbus = Eventbus::new();

    let mut collector = Collector::new(eventbus.publish());
    let mut view = View::new(eventbus.subscribe());
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

    let app = Router::new()
        .route("/hello", get(RestClient::get_latest_event))
        .with_state(rest_client.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();

    let _ = tokio::join!(collector_task, view_task, rest_client_task);
}
