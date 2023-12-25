use crate::emu_state::debug_options::DebugOptions;
use crate::emu_state::emulation::EmulationState;


pub struct AppState {
    pub debug_options: DebugOptions,
    pub emulation_state: EmulationState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            debug_options: DebugOptions::new(),
            emulation_state: EmulationState::new(),
        }
    }
}


impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}