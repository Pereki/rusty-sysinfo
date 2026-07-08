use crate::model::{cpu_info::CpuInfo, memory_info::MemoryInfo};
use sysinfo::System;

pub struct Collector {}

impl Collector {
    pub fn new() -> Self {
        Collector {}
    }

    pub fn collect_memory(&self) -> MemoryInfo {
        let mut system = System::new();
        system.refresh_all();
        MemoryInfo::new(system.used_memory(), system.total_memory())
    }

    pub fn collect_cpu(&self) -> CpuInfo {
        let mut system = System::new();
        system.refresh_all();
        CpuInfo::new(system.global_cpu_usage())
    }
}
