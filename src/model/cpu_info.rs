use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct CpuInfo {
    pub percentage: f32,
}

impl CpuInfo {
    pub fn new(percentage: f32) -> Self {
        CpuInfo { percentage }
    }
}
