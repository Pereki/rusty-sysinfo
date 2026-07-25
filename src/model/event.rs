use serde::Serialize;

use crate::model::{cpu_info::CpuInfo, memory_info::MemoryInfo};

#[derive(Clone, Copy, Serialize, Debug)]
pub struct Event {
    pub event_type: EventType,
    pub memory_info: MemoryInfo,
    pub cpu_info: CpuInfo,
}

#[derive(Clone, Copy, Serialize, Debug)]
pub enum EventType {
    UPDATE,
}

impl Event {
    pub fn new(event_type: EventType, memory_info: MemoryInfo, cpu_info: CpuInfo) -> Event {
        Event {
            event_type,
            memory_info,
            cpu_info,
        }
    }

    pub fn default() -> Event {
        Event::new(EventType::UPDATE, MemoryInfo::default(), CpuInfo::default())
    }
}
