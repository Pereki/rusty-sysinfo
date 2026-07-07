mod collector;
mod controller;
mod model;
mod view;

use crate::collector::collector::Collector;
use crate::controller::controller::Controller;
use crate::view::view::View;

fn main() {
    let collector = Collector::new();
    let view = View::new();
    let controller = Controller::new(view, collector);

    controller.boot_up();
}
