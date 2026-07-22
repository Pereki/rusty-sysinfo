use std::sync::Arc;

use crate::model::{cpu_info::CpuInfo, event::Event, memory_info::MemoryInfo};
use axum::{Json, extract::State};
use tokio::sync::{RwLock, broadcast::Receiver};

#[derive(Clone)]
pub struct RestClient {
    pub current_event_state: Arc<RwLock<Event>>,
}

impl RestClient {
    pub fn new(current_event_state: Arc<RwLock<Event>>) -> Self {
        Self {
            current_event_state,
        }
    }

    pub async fn receive(&self, receiver: &mut Receiver<Event>) {
        loop {
            match receiver.recv().await {
                Ok(el) => *self.current_event_state.write().await = el,
                Err(_) => println!("Error fetching"),
            }
        }
    }

    pub async fn get_latest_event(State(rest_client): State<Arc<RestClient>>) -> Json<Event> {
        Json(rest_client.current_event_state.read().await.clone())
    }
}
