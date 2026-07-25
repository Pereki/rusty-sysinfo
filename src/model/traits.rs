use tokio::sync::broadcast::Receiver;

use crate::model::event::Event;

pub trait AsyncSender {
    async fn run(&mut self);
}
pub trait AsyncReceiver {
    async fn receive(&mut self, receiver: &mut Receiver<Event>);
}
