mod collector;
mod eventbus;
mod model;
mod view;

use tokio::sync::broadcast;

use crate::collector::collector::Collector;
use crate::model::event::Event;
use crate::view::view::View;

#[tokio::main]
async fn main() {
    let (rx, _) = broadcast::channel::<Event>(255);

    let mut collector = Collector::new(rx.clone());
    let mut view = View::new(rx.subscribe());

    let collector_task = tokio::spawn(async move { collector.start().await });
    let view_task = tokio::spawn(async move { view.listen().await });

    let _ = tokio::join!(collector_task, view_task);
}
