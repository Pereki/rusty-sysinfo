use crate::model::{cpu_info::CpuInfo, memory_info::MemoryInfo};

pub struct View {}

impl View {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, memory_info: MemoryInfo, cpu_info: CpuInfo) {
        let used_memory_in_megabyte = memory_info.used_memory / 1024;
        let total_memory_in_megabyte = memory_info.total_memory / 1024;
        let percentage = match total_memory_in_megabyte {
            0 => 0.00,
            _ => used_memory_in_megabyte as f64 / total_memory_in_megabyte as f64 * 100.00,
        };

        println!(
            "Memory used: {} MB / {} MB ({:.2}%)",
            used_memory_in_megabyte, total_memory_in_megabyte, percentage
        );

        println!("CPU Usage: {:.2}%", cpu_info.percentage);
    }
}
