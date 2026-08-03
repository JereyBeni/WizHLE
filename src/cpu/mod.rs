pub mod arm;
pub mod x86;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    Arm,
    Arm64,
    X86,
    X86_64,
}

pub struct CpuCore {
    pub arch: Architecture,
    // Future: registers, pipeline state, etc.
}

impl CpuCore {
    pub fn new(arch: Architecture) -> Self {
        log::info!("Initializing CPU core for architecture: {:?}", arch);
        Self { arch }
    }

    pub fn reset(&mut self) {
        log::debug!("CPU reset requested");
    }
}
