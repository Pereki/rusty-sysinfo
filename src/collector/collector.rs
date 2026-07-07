use crate::model::memory_info::MemoryInfo;
use sysinfo::System;

pub struct Collector {}

impl Collector {
    pub fn new() -> Self {
        Collector {}
    }

    pub fn collect_memory(&self) -> MemoryInfo {
        let system = System::new();

        MemoryInfo::new(system.total_memory())
    }
}
