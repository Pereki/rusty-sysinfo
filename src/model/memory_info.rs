pub struct MemoryInfo {
    pub memory_usage: u64,
}

impl MemoryInfo {
    pub fn new(memory_usage: u64) -> MemoryInfo {
        MemoryInfo { memory_usage }
    }
}
