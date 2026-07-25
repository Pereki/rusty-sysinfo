use tokio::sync::broadcast::{self, Receiver, Sender};

use crate::model::event::Event;

pub struct Eventbus {
    tx: tokio::sync::broadcast::Sender<Event>,
}

impl Eventbus {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(255);
        Eventbus { tx }
    }

    pub fn subscribe(&self) -> Receiver<Event> {
        self.tx.subscribe()
    }

    pub fn publish(&self) -> Sender<Event> {
        self.tx.clone()
    }
}
