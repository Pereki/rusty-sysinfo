use serde::Serialize;

#[derive(Clone, Copy, Serialize, Debug)]
pub struct CpuInfo {
    pub percentage: f32,
}

impl CpuInfo {
    pub fn new(percentage: f32) -> Self {
        CpuInfo { percentage }
    }

    pub fn default() -> CpuInfo {
        CpuInfo::new(0.0)
    }
}
