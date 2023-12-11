pub struct DebugOptions {
    pub enable_debugging: bool,
    pub show_debug_options_window: bool,
}

impl DebugOptions {
    pub fn new() -> Self {
        Self {
            enable_debugging: true,
            show_debug_options_window: true,
        }
    }
}