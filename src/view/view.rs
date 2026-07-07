use crate::model::memory_info::MemoryInfo;

pub struct View {}

impl View {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, memory_info: MemoryInfo) {
        let used_memory_in_megabyte = memory_info.used_memory / 1024;
        let total_memory_in_megabyte = memory_info.total_memory / 1024;
        let percentage = used_memory_in_megabyte as f64 / total_memory_in_megabyte as f64 * 100.00;

        println!(
            "Memory used: {} MB / {} MB ({:.2}%)",
            used_memory_in_megabyte, total_memory_in_megabyte, percentage
        );
    }
}
