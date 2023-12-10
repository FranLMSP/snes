use eframe::egui;
use snes_core::emulator::Emulator;

pub mod ui;


#[derive(Default)]
struct SnesEmulatorApp {
    emulator: Emulator,
}

impl SnesEmulatorApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for SnesEmulatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui::menu::build_menu_bar(ui);
            ui.separator();
            // ui::game::build_game_window(ctx, ui);
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "SNES Emulator",
        native_options,
        Box::new(|cc| Box::new(SnesEmulatorApp::new(cc))),
    )
}
