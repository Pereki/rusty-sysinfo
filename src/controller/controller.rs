use crate::collector::collector::Collector;
use crate::view::view::View;

pub struct Controller {
    view: View,
    collector: Collector,
}

impl Controller {
    pub fn new(view: View, collector: Collector) -> Self {
        Controller { view, collector }
    }

    pub fn boot_up(&mut self) {
        self.collector.refresh_all();
        self.view.render(
            self.collector.collect_memory(),
            self.collector.collect_cpu(),
        );
    }
}
