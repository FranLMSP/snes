use crate::emu_state::debug_options::DebugOptions;


pub struct AppState {
    pub debug_options: DebugOptions,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            debug_options: DebugOptions::new(),
        }
    }
}


impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}