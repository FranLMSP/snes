use crate::emu_state::debug_options::DebugOptions;
use crate::emu_state::emulation::EmulationState;
use eframe::epaint::TextureHandle;

pub struct AppState {
    pub debug_options: DebugOptions,
    pub emulation_state: EmulationState,
    pub game_tv_texture: Option<TextureHandle>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            debug_options: DebugOptions::new(),
            emulation_state: EmulationState::new(),
            game_tv_texture: None,
        }
    }
}


impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}