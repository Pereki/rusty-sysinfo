use crate::model::{cpu_info::CpuInfo, memory_info::MemoryInfo};
use sysinfo::System;

pub struct Collector {
    system: System,
}

impl Collector {
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_all();

        Collector { system }
    }

    pub fn collect_memory(&self) -> MemoryInfo {
        MemoryInfo::new(self.system.used_memory(), self.system.total_memory())
    }

    pub fn collect_cpu(&self) -> CpuInfo {
        CpuInfo::new(self.system.global_cpu_usage())
    }

    pub fn refresh_all(&mut self) {
        self.system.refresh_all();
    }
}
