pub struct MemoryInfo {
    pub used_memory: u64,
    pub total_memory: u64,
}

impl MemoryInfo {
    pub fn new(used_memory: u64, total_memory: u64) -> MemoryInfo {
        MemoryInfo {
            used_memory,
            total_memory,
        }
    }
}
