extern crate snes_core;
use snes_core::ppu::registers::{
    MAX_BG_WIDTH,
    MAX_BG_HEIGHT,
    PPURegisters,
    Background as PPUBg,
};
use imgui_wgpu::Renderer;
use imgui::TextureId;
use wgpu::{Device, Queue};

use super::state::{BgDebug, VRAMMap};


pub fn background_texture(
    device: &Device,
    renderer: &mut Renderer,
    background: PPUBg
) -> TextureId {
    let texture = imgui_wgpu::Texture::new(
        device,
        renderer,
        imgui_wgpu::TextureConfig {
            label: Some(&format!("Background {:?} texture", background)),
            size: wgpu::Extent3d {
                width: MAX_BG_WIDTH as u32,
                height: MAX_BG_HEIGHT as u32,
                depth_or_array_layers: 1,
            },
            format: Some(wgpu::TextureFormat::Rgba8Unorm),
            ..imgui_wgpu::TextureConfig::default()
        },
    );
    renderer.textures.insert(texture)
}

fn render_background(bgdebug: &mut BgDebug, registers: &PPURegisters) {
    let (tile_width, tile_height) = registers.get_bg_tile_size(bgdebug.background).to_usize();
    let (bg_width, bg_height) = registers.get_bg_size(bgdebug.background).to_usize();
    let width = tile_width * bg_width;
    let height = tile_height * bg_height;
    for x in 0..width {
        for y in 0..height {
            let index = ((y * MAX_BG_WIDTH) + x) * 4;
            let r = index;
            let g = index + 1;
            let b = index + 2;
            let a = index + 3;

            bgdebug.framebuffer[r] = 0xFF;
            bgdebug.framebuffer[g] = 0xFF;
            bgdebug.framebuffer[b] = 0xFF;
            bgdebug.framebuffer[a] = 0xFF;
        }
    }
}

pub fn background_window(bgdebug: &mut BgDebug, registers: &PPURegisters, ui: &imgui::Ui, renderer: &mut Renderer, queue: &Queue) {
    if !bgdebug.is_enabled {
        return;
    }
    let texture_id = bgdebug.texture_id.unwrap();
    render_background(bgdebug, registers);

    let tex = renderer.textures.get_mut(texture_id).unwrap();
    tex.write(queue, &bgdebug.framebuffer, MAX_BG_WIDTH as u32, MAX_BG_HEIGHT as u32);
    let bg_window = imgui::Window::new(format!("BG: {:?}", bgdebug.background));
    bg_window
        .size([MAX_BG_WIDTH as f32, MAX_BG_HEIGHT as f32], imgui::Condition::FirstUseEver)
        .opened(&mut bgdebug.is_enabled)
        .build(ui, || {
            let game_image = imgui::Image::new(texture_id, [MAX_BG_WIDTH as f32, MAX_BG_HEIGHT as f32]);
            game_image.build(&ui);
        });
}

fn get_register_name(register: u16) -> &'static str {
    match register {
        snes_core::ppu::registers::INIDISP  => "INIDISP",
        snes_core::ppu::registers::TM       => "TM     ",
        snes_core::ppu::registers::TS       => "TS     ",
        snes_core::ppu::registers::SETINI   => "SETINI ",
        snes_core::ppu::registers::BGMODE   => "BGMODE ",
        snes_core::ppu::registers::MOSAIC   => "MOSAIC ",
        snes_core::ppu::registers::BG1SC    => "BG1SC  ",
        snes_core::ppu::registers::BG2SC    => "BG2SC  ",
        snes_core::ppu::registers::BG3SC    => "BG3SC  ",
        snes_core::ppu::registers::BG4SC    => "BG4SC  ",
        snes_core::ppu::registers::BG12NBA  => "BG12NBA",
        snes_core::ppu::registers::BG34NBA  => "BG34NBA",
        snes_core::ppu::registers::BG1HOFS  => "BG1HOFS",
        snes_core::ppu::registers::BG2HOFS  => "BG2HOFS",
        snes_core::ppu::registers::BG3HOFS  => "BG3HOFS",
        snes_core::ppu::registers::BG4HOFS  => "BG4HOFS",
        snes_core::ppu::registers::M7SEL    => "M7SEL  ",
        snes_core::ppu::registers::M7A      => "M7A    ",
        snes_core::ppu::registers::M7B      => "M7B    ",
        snes_core::ppu::registers::M7C      => "M7C    ",
        snes_core::ppu::registers::M7D      => "M7D    ",
        snes_core::ppu::registers::M7X      => "M7X    ",
        snes_core::ppu::registers::M7Y      => "M7Y    ",
        snes_core::ppu::registers::OBSEL    => "OBSEL  ",
        snes_core::ppu::registers::WH0      => "WH0    ",
        snes_core::ppu::registers::WH1      => "WH1    ",
        snes_core::ppu::registers::WH2      => "WH2    ",
        snes_core::ppu::registers::WH3      => "WH3    ",
        snes_core::ppu::registers::W12SEL   => "W12SEL ",
        snes_core::ppu::registers::W34SEL   => "W34SEL ",
        snes_core::ppu::registers::WOBJSEL  => "WOBJSEL",
        snes_core::ppu::registers::WBGLOG   => "WBGLOG ",
        snes_core::ppu::registers::WOBJLOG  => "WOBJLOG",
        snes_core::ppu::registers::TMW      => "TMW    ",
        snes_core::ppu::registers::TSW      => "TSW    ",
        snes_core::ppu::registers::RDNMI    => "RDNMI  ",
        snes_core::ppu::registers::VMAIN    => "VMAIN  ",
        snes_core::ppu::registers::VMADDH   => "VMADDH ",
        snes_core::ppu::registers::VMDATAL  => "VMDATAL",
        snes_core::ppu::registers::VMDATAH  => "VMDATAH",
        snes_core::ppu::registers::RDVRAML  => "RDVRAML",
        snes_core::ppu::registers::RDVRAMH  => "RDVRAMH",
        _                                   => "N/A    ",
    }
}

pub fn registers_window(ppu_registers: &PPURegisters, show_registers: &mut bool, ui: &imgui::Ui) {
    let window = imgui::Window::new("PPU Registers");
    window
        .position([250.0, 480.0], imgui::Condition::FirstUseEver)
        .size([230.0, 310.0], imgui::Condition::FirstUseEver)
        .opened(show_registers)
        .build(ui, || {
            ui.text("Registers:");
            for (index, register_value) in ppu_registers.registers().iter().enumerate() {
                let register = (index as u16) + 0x2100;
                let register_name = get_register_name(register);
                ui.text(format!("{:04X} {}: {:02X} {:08b}", register, register_name, register_value, register_value));
            }
        });
}

pub fn vram_window(ppu_registers: &PPURegisters, vram_debug: &mut VRAMMap, show_vram: &mut bool, ui: &imgui::Ui) {
    let window = imgui::Window::new("VRAM");
    window
        .size([450.0, 435.0], imgui::Condition::FirstUseEver)
        .position([500.0, 530.0], imgui::Condition::FirstUseEver)
        .opened(show_vram)
        .build(ui, || {
                let address_start_input = ui.input_text(
                    "Address start",
                    &mut vram_debug.address_start_input
                );
                address_start_input.build();

                let address_end_input = ui.input_text(
                    "Address end",
                    &mut vram_debug.address_end_input,
                );
                address_end_input.build();

                if ui.button("Apply") {
                    vram_debug.set_values_from_inputs();
                }

                ui.separator();
                let address_start = vram_debug.address_start;
                let address_end = vram_debug.address_end;
                let mut header = String::from("     | ");
                for page in 0x00..=0x0F {
                    header = format!("{}{:02X} ", header, page);
                }
                ui.text(header);
                let mut divider = String::from("-----|-");
                for _ in 0x00..=0x0F {
                    divider = format!("{}---", divider);
                }
                ui.text(divider);
                let vector = (address_start..=address_end).collect::<Vec<u16>>();
                let chunks = vector.chunks(0x10);
                for row in chunks {
                    let mut address_row = format!("{:04X} | ", row[0]);
                    for address in row {
                        address_row = format!("{}{:02X} ", address_row, ppu_registers.vram()[(*address) as usize]);
                    }
                    ui.text(address_row);
                }
        });
}
