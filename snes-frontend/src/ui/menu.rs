use eframe::egui;


pub fn build_menu_bar(ui: &mut egui::Ui) {
    egui::menu::bar(ui, |ui| {
        ui.menu_button("Emulator", |ui| {
            if ui.button("Load ROM file").clicked() {
            }
        });
        ui.menu_button("Debug", |ui| {
            if ui.button("Show Debug Menu").clicked() {
            }
        });
    });
}
