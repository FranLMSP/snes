use eframe::egui;

use crate::emu_state::AppState;


pub fn build_debug_window(ctx: &egui::Context, app_state: &mut AppState) {
    egui::Window::new("Debug")
        .open(&mut app_state.debug_options.show_debug_options_window)
        .show(ctx, |ui| {
            ui.checkbox(
                &mut app_state.debug_options.enable_debugging,
                "Enable debugging",
            );
        });
}