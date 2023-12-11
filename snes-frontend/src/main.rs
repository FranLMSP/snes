use eframe::egui;
use snes_core::emulator::Emulator;

mod emu_ui;
mod emu_state;


#[derive(Default)]
struct SnesEmulatorApp {
    emulator: Emulator,
    state: emu_state::AppState,
}

impl SnesEmulatorApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for SnesEmulatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            emu_ui::menu::build_menu_bar(&mut self.emulator, ui, &mut self.state);
            ui.separator();
            // ui::game::build_game_window(ctx);
        });
        emu_ui::debug::build_debug_window(ctx, &mut self.state);
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
