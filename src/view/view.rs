use crate::model::memory_info::MemoryInfo;

pub struct View {}

impl View {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, memory_info: MemoryInfo) {
        println!("{}", memory_info.memory_usage);
    }
}
