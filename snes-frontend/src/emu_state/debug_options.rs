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

pub struct DebugOptions {
    pub enable_debugging: bool,
    pub show_debug_options_window: bool,
    pub show_memory_map: bool,
    pub memory_map_inputs: MemoryMapInputs,
}

impl DebugOptions {
    pub fn new() -> Self {
        Self {
            enable_debugging: true,
            show_debug_options_window: true,
            show_memory_map: true,
            memory_map_inputs: MemoryMapInputs::new(),
        }
    }
}
