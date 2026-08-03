mod cpu;
mod memory;
mod graphics;
mod input;
mod touchwiz;

use eframe::egui;
use log::{info, LevelFilter};

struct WizHleApp {
    touchwiz: touchwiz::TouchWizLayer,
}

impl WizHleApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            touchwiz: touchwiz::TouchWizLayer::new(),
        }
    }
}

impl eframe::App for WizHleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.touchwiz.ui(ctx);
    }
}

fn main() -> eframe::Result<()> {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    info!("WizHLE - Experimental TouchWiz High-Level Emulator");
    info!("Starting TouchWiz UI prototype");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 780.0])   // Phone-like aspect ratio
            .with_title("WizHLE – TouchWiz"),
        ..Default::default()
    };

    eframe::run_native(
        "WizHLE",
        options,
        Box::new(|cc| Ok(Box::new(WizHleApp::new(cc)))),
    )
}
