use eframe::egui;


pub fn build_game_window(ctx: &egui::Context) {
    egui::Window::new("Game").show(ctx, |ui| {
        ui.label("Hello World!");
        render_framebuffer(ctx, ui, &[]);
    });
}


fn render_framebuffer(ctx: &egui::Context, ui: &mut egui::Ui, _framebuffer: &[u8]) {
    ui.painter().rect_filled(
        ctx.available_rect(),
        egui::Rounding::ZERO,
        egui::Color32::WHITE,
    )
}
