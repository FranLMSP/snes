use eframe::egui;
use eframe::{egui::Context, epaint::ColorImage};
use eframe::egui::{TextureOptions, Ui};
use eframe::epaint::{Vec2, TextureHandle};
use snes_core::ppu::registers::{
    MAX_TV_WIDTH,
    MAX_TV_HEIGHT,
};


pub fn build_game_window(
    ctx: &egui::Context,
    game_tv_texture: &mut Option<TextureHandle>,
    framebuffer: &[u8],
) {
    egui::Window::new("TV")
        .collapsible(false)
        .show(ctx, |ui| {
            paint_game_texture(ui, game_tv_texture, framebuffer);
        });
}

pub fn initialize_game_texture(ctx: &Context, game_tv_texture: &mut Option<TextureHandle>) {
    if game_tv_texture.is_none() {
        println!("Initializing Game TV texture");
        let _ = game_tv_texture.insert(
            ctx.load_texture(
                "Game TV texture",
                ColorImage::default(),
                Default::default(),
            )
        );
    }
}

fn paint_game_texture(ui: &mut Ui, texture: &mut Option<TextureHandle>, framebuffer: &[u8]) {
    if let Some(txt) = texture {
        txt.set(
            ColorImage::from_rgba_premultiplied([MAX_TV_WIDTH, MAX_TV_HEIGHT], framebuffer),
            TextureOptions::default(),
        );
        let (whole_rect, _) =
            ui.allocate_exact_size(
                Vec2::from([(MAX_TV_WIDTH) as f32, (MAX_TV_HEIGHT) as f32]),
                egui::Sense::focusable_noninteractive(),
            );
        egui::Image::new((
            txt.id(),
            txt.size_vec2(),
        ))
        .paint_at(ui, whole_rect);
    }
}
