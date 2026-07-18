use crate::model::{
    cpu_info::{self, CpuInfo},
    memory_info::{self, MemoryInfo},
};

#[derive(Clone)]
pub struct Event {
    pub event_type: EventType,
    pub memory_info: MemoryInfo,
    pub cpu_info: CpuInfo,
}

#[derive(Clone)]
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
}
