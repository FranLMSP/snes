use eframe::epaint::{Vec2, TextureHandle};
use eframe::{egui::Context, epaint::ColorImage};

use eframe::egui::{self, TextureOptions, Ui};
use snes_core::ppu::registers::{PPURegisters, Background, MAX_BG_WIDTH, MAX_BG_HEIGHT};
use crate::emu_state::debug_options::BgDebug;


pub fn build_bg_preview_windows(ctx: &Context, background_debug_list: &mut [BgDebug; 4], registers: &PPURegisters) {
    for bgdebug in background_debug_list {
        build_bg_preview_window(ctx, bgdebug, registers);
    }
}

fn build_bg_preview_window(ctx: &Context, bgdebug: &mut BgDebug, registers: &PPURegisters) {
    if !bgdebug.is_enabled {
        return;
    }
    initialize_bg_texture(ctx, bgdebug);
    egui::Window::new("BG Background Preview")
        .auto_sized()
        .open(&mut bgdebug.is_enabled)
        .show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.label("Charset");
                compute_2bpp_bg_char_framebuffer(bgdebug.background, &mut bgdebug.char_framebuffer, registers);
                paint_texture(ui, &mut bgdebug.char_texture, &bgdebug.char_framebuffer, 16 * 8, 8 * 8);
                ui.label("Background");
                compute_2bpp_bg_background_framebuffer(bgdebug.background, &mut bgdebug.bg_framebuffer, registers);
                paint_texture(ui, &mut bgdebug.bg_texture, &bgdebug.bg_framebuffer, MAX_BG_WIDTH, MAX_BG_HEIGHT);
            });
        });
}

fn compute_2bpp_bg_char_framebuffer(background: Background, framebuffer: &mut [u8], registers: &PPURegisters) {
    let vram_base_address = registers.get_bg_char_base_address(background) as usize;
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

            framebuffer[a] = 0xFF;
            match effective_pixel {
                0b00 => {
                    framebuffer[r] = 0xFF;
                    framebuffer[g] = 0xFF;
                    framebuffer[b] = 0xFF;
                },
                0b01 => {
                    framebuffer[r] = 0xAC;
                    framebuffer[g] = 0xAC;
                    framebuffer[b] = 0xAC;
                },
                0b10 => {
                    framebuffer[r] = 0x56;
                    framebuffer[g] = 0x56;
                    framebuffer[b] = 0x56;
                },
                0b11 => {
                    framebuffer[r] = 0x00;
                    framebuffer[g] = 0x00;
                    framebuffer[b] = 0x00;
                },
                _ => unreachable!(),
            };
        }
    }
}

fn compute_2bpp_bg_background_framebuffer(background: Background, framebuffer: &mut [u8], registers: &PPURegisters) {
    let tileset_vram_base_address = registers.get_bg_tile_base_address(background) as usize;
    let charset_vram_base_address = registers.get_bg_char_base_address(background) as usize;
    let (bg_size_width, bg_size_height) = registers.get_bg_size(background).to_usize();
    let (tile_size_width, tile_size_height) = registers.get_bg_tile_size(background).to_usize();
    let vram = registers.vram();
    let width: usize = bg_size_width * tile_size_width;
    let height: usize = bg_size_height * tile_size_height;
    for y in 0..height {
        for x in 0..width {
            let current_tile = (x / tile_size_width) + ((y / tile_size_height) * bg_size_width);
            let tile_byte = vram[tileset_vram_base_address + current_tile];
            let char_index = tile_byte & 0b11_11111111;
            let current_char_column = x.rem_euclid(tile_size_width);
            let current_char_row = y.rem_euclid(tile_size_height);
            // 8x8 pixels, 2 bitplanes, each word (16bit) holds 8 pixels
            // so 1 char is 8 bytes x 2
            let effective_vram_address =
                charset_vram_base_address +
                ((char_index as usize) * tile_size_width) +
                current_char_row;

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

            let current_fb_pixel = x + (y * MAX_BG_WIDTH);
            let fb_index = current_fb_pixel * 4;
            let r = fb_index;
            let g = fb_index + 1;
            let b = fb_index + 2;
            let a = fb_index + 3;

            framebuffer[a] = 0xFF;
            match effective_pixel {
                0b00 => {
                    framebuffer[r] = 0xFF;
                    framebuffer[g] = 0xFF;
                    framebuffer[b] = 0xFF;
                },
                0b01 => {
                    framebuffer[r] = 0xAC;
                    framebuffer[g] = 0xAC;
                    framebuffer[b] = 0xAC;
                },
                0b10 => {
                    framebuffer[r] = 0x56;
                    framebuffer[g] = 0x56;
                    framebuffer[b] = 0x56;
                },
                0b11 => {
                    framebuffer[r] = 0x00;
                    framebuffer[g] = 0x00;
                    framebuffer[b] = 0x00;
                },
                _ => unreachable!(),
            };
        }
    }
}

fn paint_texture(ui: &mut Ui, texture: &mut Option<TextureHandle>, framebuffer: &[u8], width: usize, height: usize) {
    if let Some(txt) = texture {
        txt.set(
            ColorImage::from_rgba_premultiplied([width, height], framebuffer),
            TextureOptions::LINEAR,
        );
        let (whole_rect, _) =
            ui.allocate_exact_size(Vec2::from([(width * 2) as f32, (height * 2) as f32]), egui::Sense::focusable_noninteractive());
        egui::Image::new((
            txt.id(),
            txt.size_vec2(),
        ))
        .paint_at(ui, whole_rect);
    }
}

fn initialize_bg_texture(ctx: &Context, bgdebug: &mut BgDebug) {
    if !bgdebug.is_enabled {
        return;
    }
    if bgdebug.bg_texture.is_none() {
        println!("Initializing BG texture {:?}", bgdebug.background);
        bgdebug.bg_texture = Some(
            ctx.load_texture(
                format!("BG Texture {:?}", bgdebug.background),
                ColorImage::default(),
                Default::default(),
            )
        );
    }
    if bgdebug.char_texture.is_none() {
        println!("Initializing Char texture {:?}", bgdebug.background);
        bgdebug.char_texture = Some(
            ctx.load_texture(
                format!("Char Texture {:?}", bgdebug.background),
                ColorImage::default(),
                Default::default(),
            )
        );
    }
}
