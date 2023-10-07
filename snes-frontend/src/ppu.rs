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
    background: PPUBg,
    width: u32,
    height: u32,
    name: &str,
) -> TextureId {
    let texture = imgui_wgpu::Texture::new(
        device,
        renderer,
        imgui_wgpu::TextureConfig {
            label: Some(&format!("{} {:?} texture", name, background)),
            size: wgpu::Extent3d {
                width: width,
                height: height,
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

fn render_background_char_2bpp(bgdebug: &mut BgDebug, registers: &PPURegisters) {
    let vram_base_address = registers.get_bg_char_base_address(bgdebug.background) as usize;
    let vram = registers.vram();
    let width: usize = 8 * 16;
    let height: usize = 8 * 8;
    for y in 0..height {
        for x in 0..width {
            let current_char = (x / 8) + ((y / 8) * 16);
            let current_char_column = x.rem_euclid(8);
            let current_char_row = y.rem_euclid(8);
            // 8x8 pixels, 2 bitplanes, each word (16bit) holds 8 pixels
            // so 1 char is 8 bytes x 2
            let char_base_vram_address = current_char * 8;
            let effective_vram_address = vram_base_address + (
                char_base_vram_address + (current_char_row)
            );
            let current_pixel = x + (y * width);

            let vram_word = vram[effective_vram_address];
            let lsb_bitplane= vram_word as u8;
            let msb_bitplane= (vram_word >> 8) as u8;
            let pixels = [
                (
                    (lsb_bitplane >> 7) |
                    ((msb_bitplane >> 7) << 1)
                ),
                (
                    ((lsb_bitplane >> 6) & 1) |
                    (((msb_bitplane >> 6) & 1) << 1)
                ),
                (
                    ((lsb_bitplane >> 5) & 1) |
                    (((msb_bitplane >> 5) & 1) << 1)
                ),
                (
                    ((lsb_bitplane >> 4) & 1) |
                    (((msb_bitplane >> 4) & 1) << 1)
                ),
                (
                    ((lsb_bitplane >> 3) & 1) |
                    (((msb_bitplane >> 3) & 1) << 1)
                ),
                (
                    ((lsb_bitplane >> 2) & 1) |
                    (((msb_bitplane >> 2) & 1) << 1)
                ),
                (
                    ((lsb_bitplane >> 1) & 1) |
                    (((msb_bitplane >> 1) & 1) << 1)
                ),
                (
                    (lsb_bitplane & 1) |
                    ((msb_bitplane & 1) << 1)
                ),
            ];
            let effective_pixel = pixels[current_char_column];

            let fb_index = current_pixel * 4;
            let r = fb_index;
            let g = fb_index + 1;
            let b = fb_index + 2;
            let a = fb_index + 3;

            bgdebug.char_framebuffer[a] = 0xFF;
            match effective_pixel {
                0b00 => {
                    bgdebug.char_framebuffer[r] = 0xFF;
                    bgdebug.char_framebuffer[g] = 0xFF;
                    bgdebug.char_framebuffer[b] = 0xFF;
                },
                0b01 => {
                    bgdebug.char_framebuffer[r] = 0xAC;
                    bgdebug.char_framebuffer[g] = 0xAC;
                    bgdebug.char_framebuffer[b] = 0xAC;
                },
                0b10 => {
                    bgdebug.char_framebuffer[r] = 0x56;
                    bgdebug.char_framebuffer[g] = 0x56;
                    bgdebug.char_framebuffer[b] = 0x56;
                },
                0b11 => {
                    bgdebug.char_framebuffer[r] = 0x00;
                    bgdebug.char_framebuffer[g] = 0x00;
                    bgdebug.char_framebuffer[b] = 0x00;
                },
                _ => unreachable!(),
            };
        }
    }
}

pub fn background_window(bgdebug: &mut BgDebug, registers: &PPURegisters, ui: &imgui::Ui, renderer: &mut Renderer, queue: &Queue) {
    if !bgdebug.is_enabled {
        return;
    }
    let texture_id = bgdebug.texture_id.unwrap();
    let char_texture_id = bgdebug.char_texture_id.unwrap();
    render_background(bgdebug, registers);
    render_background_char_2bpp(bgdebug, registers);

    let tex = renderer.textures.get_mut(texture_id).unwrap();
    tex.write(queue, &bgdebug.framebuffer, MAX_BG_WIDTH as u32, MAX_BG_HEIGHT as u32);
    let char_tex = renderer.textures.get_mut(char_texture_id).unwrap();
    char_tex.write(queue, &bgdebug.char_framebuffer, 16 * 8, 8 * 8);
    let bg_window = imgui::Window::new(format!("BG: {:?}", bgdebug.background));
    bg_window
        .size([MAX_BG_WIDTH as f32, MAX_BG_HEIGHT as f32], imgui::Condition::FirstUseEver)
        .opened(&mut bgdebug.is_enabled)
        .build(ui, || {
            ui.text("Charset:");
            let charset_image = imgui::Image::new(char_texture_id, [16.0 * 8.0 * 2.0, 8.0 * 8.0 * 2.0]);
            charset_image.build(&ui);
            ui.separator();
            ui.text("Background:");
            let bg_image = imgui::Image::new(texture_id, [MAX_BG_WIDTH as f32, MAX_BG_HEIGHT as f32]);
            bg_image.build(&ui);
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

pub fn registers_window(ppu_registers: &PPURegisters, show_registers: &mut bool, ui: &imgui::Ui) {
    let window = imgui::Window::new("PPU Registers");
    window
        .position([250.0, 480.0], imgui::Condition::FirstUseEver)
        .size([230.0, 310.0], imgui::Condition::FirstUseEver)
        .opened(show_registers)
        .build(ui, || {
            ui.text("H/V Counters:");
            ui.text(format!("V: {}", ppu_registers.v_count));
            ui.text(format!("H: {}", ppu_registers.h_count));
            ui.text(format!("VBlanking: {}", ppu_registers.is_vblanking()));
            ui.text(format!("NMI Flag:  {}", ppu_registers.vblank_nmi));
            ui.separator();
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
                    header = format!("{}  {:02X} ", header, page);
                }
                ui.text(header);
                let mut divider = String::from("-----|-");
                for _ in 0x00..=0x0F {
                    divider = format!("{}-----", divider);
                }
                ui.text(divider);
                let vector = (address_start..=address_end).collect::<Vec<u16>>();
                let chunks = vector.chunks(0x10);
                for row in chunks {
                    let mut address_row = format!("{:04X} | ", row[0]);
                    for address in row {
                        address_row = format!("{}{:04X} ", address_row, ppu_registers.vram()[((*address) & 0x7FFF) as usize]);
                    }
                    ui.text(address_row);
                }
        });
}
