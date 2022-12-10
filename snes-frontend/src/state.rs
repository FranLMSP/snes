pub struct DebugOptions {
    pub show_debug_window: bool,
    pub show_cpu_registers: bool,
    pub show_spc700_registers: bool,
    pub show_cpu_memory: bool,
}

impl DebugOptions {
    pub fn new() -> Self {
        Self {
            show_debug_window: false,
            show_cpu_registers: true,
            show_spc700_registers: true,
            show_cpu_memory: true,
        }
    }
}

pub struct State {
    pub debug_options: DebugOptions,
}

impl State {
    pub fn new() -> Self {
        Self {
            debug_options: DebugOptions::new(),
        }
    }
}