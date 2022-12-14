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
use super::state::BgDebug;


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
    let game_window = imgui::Window::new(format!("BG: {:?}", bgdebug.background));
    game_window
        .size([MAX_BG_WIDTH as f32, MAX_BG_HEIGHT as f32], imgui::Condition::FirstUseEver)
        .opened(&mut bgdebug.is_enabled)
        .build(ui, || {
            let game_image = imgui::Image::new(texture_id, [MAX_BG_WIDTH as f32, MAX_BG_HEIGHT as f32]);
            game_image.build(&ui);
        });
}