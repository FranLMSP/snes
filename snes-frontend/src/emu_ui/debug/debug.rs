use eframe::egui;
use snes_core::emulator::Emulator;

use crate::emu_state::debug_options::DebugOptions;
use crate::emu_state::emulation::EmulationState;

use super::memory_map::build_memory_map_window;
use super::cpu::build_cpu_debug_controls;


pub fn build_all_debug_options(ctx: &egui::Context, debug_options: &mut DebugOptions, emulation_state: &mut EmulationState, emulator: &mut Emulator) {
    build_debug_options_window(ctx, debug_options);

    if !debug_options.enable_debugging {
        return;
    }

    build_memory_map_window(ctx, &mut debug_options.memory_map_conrtrol_options, emulator);
    build_cpu_debug_controls(ctx, &mut debug_options.cpu_debug_control_options, emulation_state, emulator);
}


fn build_debug_options_window(ctx: &egui::Context, debug_options: &mut DebugOptions) {
    egui::Window::new("Debug")
        .open(&mut debug_options.show_debug_options_window)
        .show(ctx, |ui| {
            ui.checkbox(
                &mut debug_options.enable_debugging,
                "Enable debugging",
            );
            ui.separator();
            if ui.selectable_label(
                debug_options.memory_map_conrtrol_options.is_enabled,
                "Show memory map"
            ).clicked() {
                debug_options.memory_map_conrtrol_options.is_enabled = !debug_options.memory_map_conrtrol_options.is_enabled;
            }
            if ui.selectable_label(
                debug_options.cpu_debug_control_options.is_enabled,
                "Show CPU Debug Controls"
            ).clicked() {
                debug_options.cpu_debug_control_options.is_enabled = !debug_options.cpu_debug_control_options.is_enabled;
            }
        });
}
