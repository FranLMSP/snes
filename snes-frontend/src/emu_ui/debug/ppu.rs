use eframe::egui;
use snes_core::ppu::registers::PPURegisters;

use crate::emu_state::debug_options::PPUDebugControlOptions;
use crate::emu_ui::debug::common::sanitize_input;


pub fn build_ppu_debug_controls(ctx: &egui::Context, ppu_debug_options: &mut PPUDebugControlOptions, ppu_registers: &PPURegisters) {
    if !ppu_debug_options.is_enabled {
        return
    }

    egui::Window::new("PPU Debug Controls")
        .auto_sized()
        .max_width(125.0)
        .open(&mut ppu_debug_options.is_enabled)
        .show(ctx, |ui| {
            ui.monospace("Controls:");
            ui.horizontal(|ui| {
                if ui.selectable_label(
                    ppu_debug_options.show_registers,
                    "Show PPU Registers"
                ).clicked() {
                    ppu_debug_options.show_registers = !ppu_debug_options.show_registers;
                }
                if ui.selectable_label(
                    ppu_debug_options.show_vram,
                    "Show VRAM"
                ).clicked() {
                    ppu_debug_options.show_vram = !ppu_debug_options.show_vram;
                }
            });
            ui.monospace("Backgrounds:");
            ui.horizontal(|ui| {
                for bgdebug in ppu_debug_options.backgrounds.iter_mut() {
                    if ui.selectable_label(
                        bgdebug.is_enabled,
                        format!("Show {:?}", bgdebug.background),
                    ).clicked() {
                        bgdebug.is_enabled = !bgdebug.is_enabled;
                    }
                }
            });
        });

    build_ppu_registers_window(ctx, ppu_debug_options, ppu_registers);
    build_vram_window(ctx, ppu_debug_options, ppu_registers);
}

fn build_ppu_registers_window(ctx: &egui::Context, ppu_debug_options: &mut PPUDebugControlOptions, ppu_registers: &PPURegisters) {
    if !ppu_debug_options.show_registers {
        return;
    }
    egui::Window::new("PPU Registers")
        .max_width(180.0)
        .open(&mut ppu_debug_options.show_registers)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.monospace(format!(
                    "H count: {} V count: {}",
                    ppu_registers.h_count,
                    ppu_registers.v_count,
                ));
                ui.monospace(format!("VBlanking: {}", ppu_registers.is_vblanking()));
                ui.monospace(format!("NMI Flag: {}", ppu_registers.vblank_nmi));
                ui.separator();
                ui.monospace("Registers:");
                for (index, register_value) in ppu_registers.registers().iter().enumerate() {
                    let register = (index as u16) + 0x2100;
                    let register_name = get_register_name(register);
                    ui.monospace(format!("{:04X} {}: {:02X} {:08b}", register, register_name, register_value, register_value));
                }
            });
        });
}

fn get_register_name(register: u16) -> &'static str {
    match register {
        snes_core::ppu::registers::INIDISP  => "INIDISP",
        snes_core::ppu::registers::OBSEL    => "OBSEL  ",
        snes_core::ppu::registers::OAMADDL  => "OAMADDL",
        snes_core::ppu::registers::OAMADDH  => "OAMADDH",
        snes_core::ppu::registers::OAMDATA  => "OAMDATA",
        snes_core::ppu::registers::BGMODE   => "BGMODE ",
        snes_core::ppu::registers::MOSAIC   => "MOSAIC ",
        snes_core::ppu::registers::BG1SC    => "BG1SC  ",
        snes_core::ppu::registers::BG2SC    => "BG2SC  ",
        snes_core::ppu::registers::BG3SC    => "BG3SC  ",
        snes_core::ppu::registers::BG4SC    => "BG4SC  ",
        snes_core::ppu::registers::BG12NBA  => "BG12NBA",
        snes_core::ppu::registers::BG34NBA  => "BG34NBA",
        snes_core::ppu::registers::BG1HOFS  => "BG1HOFS",
        snes_core::ppu::registers::BG1VOFS  => "BG1VOFS",
        snes_core::ppu::registers::BG2HOFS  => "BG2HOFS",
        snes_core::ppu::registers::BG2VOFS  => "BG2VOFS",
        snes_core::ppu::registers::BG3HOFS  => "BG3HOFS",
        snes_core::ppu::registers::BG3VOFS  => "BG3VOFS",
        snes_core::ppu::registers::BG4HOFS  => "BG4HOFS",
        snes_core::ppu::registers::BG4VOFS  => "BG4VOFS",
        snes_core::ppu::registers::VMAIN    => "VMAIN  ",
        snes_core::ppu::registers::VMADDL   => "VMADDL ",
        snes_core::ppu::registers::VMADDH   => "VMADDH ",
        snes_core::ppu::registers::VMDATAL  => "VMDATAL",
        snes_core::ppu::registers::VMDATAH  => "VMDATAH",
        snes_core::ppu::registers::M7SEL    => "M7SEL  ",
        snes_core::ppu::registers::M7A      => "M7A    ",
        snes_core::ppu::registers::M7B      => "M7B    ",
        snes_core::ppu::registers::M7C      => "M7C    ",
        snes_core::ppu::registers::M7D      => "M7D    ",
        snes_core::ppu::registers::M7X      => "M7X    ",
        snes_core::ppu::registers::M7Y      => "M7Y    ",
        snes_core::ppu::registers::CGADD    => "CGADD  ",
        snes_core::ppu::registers::CGDATA   => "CGDATA ",
        snes_core::ppu::registers::W12SEL   => "W12SEL ",
        snes_core::ppu::registers::W34SEL   => "W34SEL ",
        snes_core::ppu::registers::WOBJSEL  => "WOBJSEL",
        snes_core::ppu::registers::WH0      => "WH0    ",
        snes_core::ppu::registers::WH1      => "WH1    ",
        snes_core::ppu::registers::WH2      => "WH2    ",
        snes_core::ppu::registers::WH3      => "WH3    ",
        snes_core::ppu::registers::WBGLOG   => "WBGLOG ",
        snes_core::ppu::registers::WOBJLOG  => "WOBJLOG",
        snes_core::ppu::registers::TM       => "TM     ",
        snes_core::ppu::registers::TS       => "TS     ",
        snes_core::ppu::registers::TMW      => "TMW    ",
        snes_core::ppu::registers::TSW      => "TSW    ",
        snes_core::ppu::registers::CGWSEL   => "CGWSEL ",
        snes_core::ppu::registers::CGADSUB  => "CGADSUB",
        snes_core::ppu::registers::COLDATA  => "COLDATA",
        snes_core::ppu::registers::SETINI   => "SETINI ",
        snes_core::ppu::registers::MPYL     => "MPYL   ",
        snes_core::ppu::registers::MPYM     => "MPYM   ",
        snes_core::ppu::registers::MPYH     => "MPYH   ",
        snes_core::ppu::registers::SLHV     => "SLHV   ",
        snes_core::ppu::registers::RDOAM    => "RDOAM  ",
        snes_core::ppu::registers::RDVRAML  => "RDVRAML",
        snes_core::ppu::registers::RDVRAMH  => "RDVRAMH",
        snes_core::ppu::registers::RDCGRAM  => "RDCGRAM",
        snes_core::ppu::registers::OPHCT    => "OPHCT  ",
        snes_core::ppu::registers::OPVCT    => "OPVCT  ",
        snes_core::ppu::registers::STAT77   => "STAT77 ",
        snes_core::ppu::registers::STAT78   => "STAT78 ",
        _                                   => "N/A    ",
    }
}

fn build_vram_window(ctx: &egui::Context, ppu_debug_options: &mut PPUDebugControlOptions, ppu_registers: &PPURegisters) {
    if !ppu_debug_options.show_vram {
        return;
    }
    egui::Window::new("VRAM Viewer")
        .open(&mut ppu_debug_options.show_vram)
        .default_width(610.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Address Start: ");
                ui.text_edit_singleline(&mut ppu_debug_options.vram_inputs.address_start);
            });
            ui.horizontal(|ui| {
                ui.label("Address End: ");
                ui.text_edit_singleline(&mut ppu_debug_options.vram_inputs.address_end);
            });

            if ui.button("Search").clicked() {
                sanitize_input(&mut ppu_debug_options.vram_inputs.address_start, false);
                sanitize_input(&mut ppu_debug_options.vram_inputs.address_end, false);
                ppu_debug_options.vram_inputs_result.address_start = ppu_debug_options.vram_inputs.address_start.to_string();
                ppu_debug_options.vram_inputs_result.address_end = ppu_debug_options.vram_inputs.address_end.to_string();
            }

            ui.separator();

            egui::ScrollArea::both().show(ui, |ui| {
                let address_start = u16::from_str_radix(&ppu_debug_options.vram_inputs_result.address_start, 16).unwrap();
                let address_end = u16::from_str_radix(&ppu_debug_options.vram_inputs_result.address_end, 16).unwrap();
                let mut header = String::from("     | ");
                for page in 0x00..=0x0F {
                    header = format!("{}  {:02X} ", header, page);
                }
                ui.monospace(header);
                let mut divider = String::from("-----|-");
                for _ in 0x00..=0x0F {
                    divider = format!("{}-----", divider);
                }
                ui.monospace(divider);
                let vector = (address_start..=address_end).collect::<Vec<u16>>();
                let chunks = vector.chunks(0x10);
                for row in chunks {
                    let mut address_row = format!("{:04X} | ", row[0]);
                    for address in row {
                        address_row = format!("{}{:04X} ", address_row, ppu_registers.vram()[((*address) & 0x7FFF) as usize]);
                    }
                    ui.monospace(address_row);
                }
            });
        });
}
