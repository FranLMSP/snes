use eframe::egui;
use snes_core::{emulator::Emulator, cpu::instructions::mapper::map_opcode_to_instruction};

use crate::emu_state::{debug_options::CPUDebugControlOptions, emulation::EmulationState};


pub fn build_cpu_debug_controls(ctx: &egui::Context, cpu_debug_options: &mut CPUDebugControlOptions, emulation_state: &mut EmulationState, emulator: &mut Emulator) {
    if !cpu_debug_options.is_enabled {
        return
    }

    egui::Window::new("CPU Debug Controls")
        .auto_sized()
        .max_width(125.0)
        .open(&mut cpu_debug_options.is_enabled)
        .show(ctx, |ui| {
            ui.monospace("Controls:");
            ui.horizontal(|ui| {
                if ui.selectable_label(
                    emulation_state.one_tick_per_frame,
                    "Tick per frame"
                ).clicked() {
                    emulation_state.one_tick_per_frame = !emulation_state.one_tick_per_frame;
                }
                let pause_text = if emulation_state.is_paused {"Resume"} else {"Pause"};
                if ui.button(pause_text).clicked() {
                    emulation_state.is_paused = !emulation_state.is_paused;
                }
                let tick_button = ui.add_enabled(emulation_state.is_paused, egui::Button::new("Tick"));
                if tick_button.clicked() {
                    emulator.tick();
                }
            });
            ui.monospace("Vectors:");
            ui.horizontal(|ui| {
                if ui.button("Reset").clicked() {
                    emulator.reset();
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                if ui.selectable_label(
                    cpu_debug_options.show_registers,
                    "Show registers"
                ).clicked() {
                    cpu_debug_options.show_registers = !cpu_debug_options.show_registers;
                }
                if ui.selectable_label(
                    cpu_debug_options.show_upcoming_instruction,
                    "Show upcoming instruction"
                ).clicked() {
                    cpu_debug_options.show_upcoming_instruction = !cpu_debug_options.show_upcoming_instruction;
                }
            });
        });

    build_cpu_registers_window(ctx, cpu_debug_options, emulator);
    build_upcoming_instruction_window(ctx, cpu_debug_options, emulator);
}

fn build_cpu_registers_window(ctx: &egui::Context, cpu_debug_options: &mut CPUDebugControlOptions, emulator: &Emulator) {
    egui::Window::new("CPU Registers")
        .auto_sized()
        .max_width(125.0)
        .open(&mut cpu_debug_options.show_registers)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.monospace(format!("SP:       | {:#06X}",   emulator.cpu.registers.sp));
                ui.monospace(format!("X:        | {:#06X}",   emulator.cpu.registers.x));
                ui.monospace(format!("Y:        | {:#06X}",   emulator.cpu.registers.y));
                ui.monospace(format!("A:        | {:#06X}",   emulator.cpu.registers.a));
                ui.monospace(format!("P:        |   {:#04X}", emulator.cpu.registers.p));
                ui.monospace(format!("D:        | {:#06X}",   emulator.cpu.registers.d));
                ui.monospace(format!("PBR:      |   {:#04X}", emulator.cpu.registers.pbr));
                ui.monospace(format!("DBR:      |   {:#04X}", emulator.cpu.registers.dbr));
                ui.monospace(format!("PC:       | {:#06X}",   emulator.cpu.registers.pc));
                ui.monospace(format!("EMU MODE: |  {}",       emulator.cpu.registers.emulation_mode));
                ui.separator();
                ui.monospace("Status flags:");
                ui.monospace("NVMXDIZC");
                ui.monospace(format!("{:08b}", emulator.cpu.registers.p));
            });
        });
}

fn build_upcoming_instruction_window(ctx: &egui::Context, cpu_debug_options: &mut CPUDebugControlOptions, emulator: &Emulator) {
    egui::Window::new("Upcoming CPU Instruction")
        .auto_sized()
        .min_width(150.0)
        .open(&mut cpu_debug_options.show_upcoming_instruction)
        .show(ctx, |ui| {
            let opcode = emulator.bus.read_external(emulator.cpu.registers.get_pc_address());
            let instruction = map_opcode_to_instruction(opcode);
            ui.monospace(
                instruction.mnemonic(&emulator.cpu.registers, &emulator.bus, opcode)
            );
        });
}
