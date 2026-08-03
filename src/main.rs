mod cpu;
mod memory;
mod graphics;
mod input;
mod touchwiz;

use log::{info, LevelFilter};

fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    info!("WizHLE - Experimental TouchWiz High-Level Emulator");
    info!("Architecture support: ARM and x86 (planned)");
    info!("This is currently a non-functional skeleton project.");

    // Placeholder: initialize subsystems
    let _cpu = cpu::CpuCore::new(cpu::Architecture::Arm);
    let _mem = memory::MemoryManager::new(512 * 1024 * 1024); // 512 MB placeholder
    let _gfx = graphics::GraphicsBackend::new();
    let _input = input::InputHandler::new();
    let _tw = touchwiz::TouchWizLayer::new();

    info!("Skeleton initialized. No real emulation is performed yet.");
}
