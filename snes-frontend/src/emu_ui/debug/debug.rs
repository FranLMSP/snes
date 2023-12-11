use eframe::egui;
use snes_core::emulator::Emulator;

use crate::emu_state::AppState;

use super::memory_map::build_memory_map_window;


fn build_debug_options_window(ctx: &egui::Context, app_state: &mut AppState) {
    egui::Window::new("Debug")
        .open(&mut app_state.debug_options.show_debug_options_window)
        .show(ctx, |ui| {
            ui.checkbox(
                &mut app_state.debug_options.enable_debugging,
                "Enable debugging",
            );
            ui.separator();
            if ui.selectable_label(
                app_state.debug_options.show_memory_map,
                "Show memory map"
            ).clicked() {
                app_state.debug_options.show_memory_map = !app_state.debug_options.show_memory_map;
            }
        });
}

pub fn build_all_debug_options(ctx: &egui::Context, app_state: &mut AppState, emulator: &Emulator) {
    build_debug_options_window(ctx, app_state);

    if !app_state.debug_options.enable_debugging {
        return;
    }

    build_memory_map_window(ctx, app_state, emulator);
}
