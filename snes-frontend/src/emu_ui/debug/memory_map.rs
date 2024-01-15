use eframe::egui;
use snes_core::emulator::Emulator;
use crate::emu_ui::debug::common::sanitize_input;

use crate::emu_state::debug_options::{MemoryMapInputs, MemoryMapControlOptions};


pub fn build_memory_map_window(ctx: &egui::Context, memory_map_control_options: &mut MemoryMapControlOptions, emulator: &Emulator) {
    if !memory_map_control_options.is_enabled {
        return
    }

    egui::Window::new("Memory Map")
        .min_size([400.0, 400.0])
        .open(&mut memory_map_control_options.is_enabled)
        .show(ctx, |ui| {
            build_inputs(
                ui,
                &mut memory_map_control_options.inputs,
                &mut memory_map_control_options.inputs_result,
            );
            ui.separator();
            egui::ScrollArea::both().show(ui, |ui| {
                build_memory_map_text(
                    ui,
                    &memory_map_control_options.inputs_result,
                    emulator,
                );
            });
        });
}

fn build_inputs(ui: &mut egui::Ui, input_values: &mut MemoryMapInputs, input_result: &mut MemoryMapInputs) {
    ui.horizontal(|ui| {
        ui.label("Page Start: ");
        ui.text_edit_singleline(&mut input_values.page_start);
    });
    ui.horizontal(|ui| {
        ui.label("Page End: ");
        ui.text_edit_singleline(&mut input_values.page_end);
    });
    ui.horizontal(|ui| {
        ui.label("Address Start: ");
        ui.text_edit_singleline(&mut input_values.address_start);
    });
    ui.horizontal(|ui| {
        ui.label("Address End: ");
        ui.text_edit_singleline(&mut input_values.address_end);
    });
    if ui.button("Search").clicked() {
        sanitize_input(&mut input_values.page_start, true);
        sanitize_input(&mut input_values.page_end, true);
        sanitize_input(&mut input_values.address_start, false);
        sanitize_input(&mut input_values.address_end, false);
        input_result.page_start = format!("{}", input_values.page_start);
        input_result.page_end = format!("{}", input_values.page_end);
        input_result.address_start = format!("{}", input_values.address_start);
        input_result.address_end = format!("{}", input_values.address_end);
    }
}

fn build_memory_map_text(ui: &mut egui::Ui, input_values: &MemoryMapInputs, emulator: &Emulator) {
    let page_start = u8::from_str_radix(&input_values.page_start, 16).unwrap();
    let page_end = u8::from_str_radix(&input_values.page_end, 16).unwrap();
    let address_start = u16::from_str_radix(&input_values.address_start, 16).unwrap();
    let address_end = u16::from_str_radix(&input_values.address_end, 16).unwrap();

    let mut header = String::from("     | ");
    for page in (page_start..=page_end).rev() {
        header = format!("{}{:02X} ", header, page);
    }
    ui.monospace(header);
    let mut divider = String::from("-----|-");
    for _page in (page_start..=page_end).rev() {
        divider = format!("{}---", divider);
    }
    ui.monospace(divider);
    for address in (address_start..=address_end).rev() {
        let mut address_row = format!("{:04X} | ", address);
        for page in (page_start..=page_end).rev() {
            let bus_address = ((page as u32) << 16) | (address as u32);
            address_row = format!("{}{:02X} ", address_row, emulator.bus.read_external(bus_address));
        }
        ui.monospace(address_row);
    }
}
