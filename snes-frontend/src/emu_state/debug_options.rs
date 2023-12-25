pub struct MemoryMapControlOptions {
    pub is_enabled: bool,
    pub inputs: MemoryMapInputs,
}

impl MemoryMapControlOptions {
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            inputs: MemoryMapInputs::new(),
        }
    }
}

pub struct MemoryMapInputs {
    pub page_start: String,
    pub page_end: String,
    pub address_start: String,
    pub address_end: String,
}

impl MemoryMapInputs {
    pub fn new() -> Self {
        Self {
            page_start: String::from("00"),
            page_end: String::from("0F"),
            address_start: String::from("0000"),
            address_end: String::from("01FF"),
        }
    }
}

pub struct CPUDebugControlOptions {
    pub is_enabled: bool,
    pub show_registers: bool,
}

impl CPUDebugControlOptions {
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            show_registers: true,
        }
    }
}

pub struct DebugOptions {
    pub enable_debugging: bool,
    pub show_debug_options_window: bool,
    pub memory_map_conrtrol_options: MemoryMapControlOptions,
    pub cpu_debug_control_options: CPUDebugControlOptions,
}

impl DebugOptions {
    pub fn new() -> Self {
        Self {
            enable_debugging: true,
            show_debug_options_window: true,
            memory_map_conrtrol_options: MemoryMapControlOptions::new(),
            cpu_debug_control_options: CPUDebugControlOptions::new(),
        }
    }
}
