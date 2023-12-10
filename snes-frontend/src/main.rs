use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("SNES Emulator", native_options, Box::new(|cc| Box::new(SnesEmulatorApp::new(cc))))
}

#[derive(Default)]
struct SnesEmulatorApp {}

impl SnesEmulatorApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for SnesEmulatorApp {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });
   }
}
