use eframe::egui;
use snes_core::emulator::Emulator;

use crate::emu_state::AppState;


pub fn build_menu_bar(emulator: &mut Emulator, ui: &mut egui::Ui, state: &mut AppState) {
    egui::menu::bar(ui, |ui| {
        ui.menu_button("Emulator", |ui| {
            if ui.button("Load ROM file").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    let picked_path = path.display().to_string();
                    match emulator.bus.rom.load(&picked_path) {
                        Ok(_) => {
                            emulator.hard_reset();
                            state.emulation_state.is_paused = false;
                            state.emulation_state.one_tick_per_frame = false;
                            emulator.reset_vector();
                            println!("Loaded ROM");
                        },
                        Err(err) => println!("Error loading the ROM: {}", err),
                    };
                }
            }
        });
        ui.menu_button("Debug", |ui| {
            if ui.button("Show Debug Menu").clicked() {
                state.debug_options.show_debug_options_window = true;
            }
        });
    });
}
