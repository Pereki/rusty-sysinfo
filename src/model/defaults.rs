use crate::model::event::Event;

pub struct Defaults {
    pub frontend_path: &str,
    pub should_run_terminal: bool,
    pub port: u16,
    pub default_event: Event,
}

impl Defaults {
    pub fn new() -> Self {
        Self {
            frontend_path: &format!("{}/frontend-dir", env!("CARGO_MANIFEST_DIR")),
            should_run_terminal: true,
            port: 80,
            default_event: Event::default(),
        }
    }
}
