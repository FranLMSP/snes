use eframe::egui;
use snes_core::emulator::Emulator;

mod utils;
mod emu_ui;
mod emu_state;


#[derive(Default)]
struct SnesEmulatorApp {
    emulator: Emulator,
    state: emu_state::AppState,
    frame_limit: utils::frame_limiter::FrameLimiter,
}

impl SnesEmulatorApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for SnesEmulatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            emu_ui::menu::build_menu_bar(&mut self.emulator, ui, &mut self.state);
            ui.separator();
        });
        emu_ui::game::initialize_game_texture(ctx, &mut self.state.game_tv_texture);
        emu_ui::game::build_game_window(
            ctx,
            &mut self.state.game_tv_texture,
            self.emulator.bus.ppu.framebuffer(),
        );
        emu_ui::debug::build_all_debug_options(ctx, &mut self.state.debug_options, &mut self.state.emulation_state, &mut self.emulator);
        if !self.state.emulation_state.is_paused {
            if self.state.emulation_state.one_tick_per_frame {
                self.emulator.tick();
            } else {
                self.emulator.loop_frame();
            }
        }
        ctx.request_repaint();
        self.frame_limit.limit();
        self.frame_limit.reset_timer();
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        vsync: false,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "SNES Emulator",
        native_options,
        Box::new(|cc| Box::new(SnesEmulatorApp::new(cc))),
    )
}
