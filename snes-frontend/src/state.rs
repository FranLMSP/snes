extern crate snes_core;
use snes_core::ppu::registers::{
    Background as PPUBg,
    MAX_BG_WIDTH,
    MAX_BG_HEIGHT,
};
use regex::Regex;


pub struct MemoryMap {
    pub is_enabled: bool,
    pub page_start: u8,
    pub page_end: u8,
    pub address_start: u16,
    pub address_end: u16,
    pub page_start_input: String,
    pub page_end_input: String,
    pub address_start_input: String,
    pub address_end_input: String,
}

impl MemoryMap {
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            page_start: 0xF0,
            page_end: 0xFF,
            address_start: 0xFFF0,
            address_end: 0xFFFF,
            page_start_input: String::from("0xF0"),
            page_end_input: String::from("0xFF"),
            address_start_input: String::from("0xFFF0"),
            address_end_input: String::from("0xFFFF"),
        }
    }

    fn validate_values(&mut self) {
        if self.page_start > self.page_end {
            self.page_start = self.page_end;
        }
        if self.page_end < self.page_start {
            self.page_end = self.page_start;
        }
        if self.address_start > self.address_end {
            self.address_start = self.address_end;
        }
        if self.address_end < self.address_start {
            self.address_end = self.address_start;
        }
    }

    pub fn set_values_from_inputs(&mut self) {
        let page_regex = Regex::new(r"^(0x)?[0-9a-fA-F]{2,2}$").unwrap();
        let address_regex = Regex::new(r"^(0x)?[0-9a-fA-F]{4,4}$").unwrap();

        if !page_regex.is_match(&self.page_start_input) {
            println!("Page start didn't match");
            self.page_start_input = String::from("0x00");
        }
        if !page_regex.is_match(&self.page_end_input) {
            println!("Page end didn't match");
            self.page_end_input = String::from("0x00");
        }
        if !address_regex.is_match(&self.address_start_input) {
            println!("Addr start didn't match");
            self.address_start_input = String::from("0x0000");
        }
        if !address_regex.is_match(&self.address_end_input) {
            println!("Addr end didn't match");
            self.address_end_input = String::from("0x0000");
        }

        self.page_start = u8::from_str_radix(&self.page_start_input.trim_start_matches("0x"), 16).unwrap();
        self.page_end = u8::from_str_radix(&self.page_end_input.trim_start_matches("0x"), 16).unwrap();
        self.address_start = u16::from_str_radix(&self.address_start_input.trim_start_matches("0x"), 16).unwrap();
        self.address_end = u16::from_str_radix(&self.address_end_input.trim_start_matches("0x"), 16).unwrap();
        self.validate_values()
    }
}

pub struct VRAMMap {
    pub address_start: u16,
    pub address_end: u16,
    pub address_start_input: String,
    pub address_end_input: String,
}

impl VRAMMap {
    pub fn new() -> Self {
        Self {
            address_start: 0x0000,
            address_end: 0x00FF,
            address_start_input: String::from("0000"),
            address_end_input: String::from("0100"),
        }
    }

    fn validate_values(&mut self) {
        if self.address_start > self.address_end {
            self.address_start = self.address_end;
        }
        if self.address_end < self.address_start {
            self.address_end = self.address_start;
        }
    }

    pub fn set_values_from_inputs(&mut self) {
        let address_regex = Regex::new(r"^(0x)?[0-9a-fA-F]{4,4}$").unwrap();

        if !address_regex.is_match(&self.address_start_input) {
            println!("Addr start didn't match");
            self.address_start_input = String::from("0x0000");
        }
        if !address_regex.is_match(&self.address_end_input) {
            println!("Addr end didn't match");
            self.address_end_input = String::from("0x0000");
        }

        self.address_start = u16::from_str_radix(&self.address_start_input.trim_start_matches("0x"), 16).unwrap();
        self.address_end = u16::from_str_radix(&self.address_end_input.trim_start_matches("0x"), 16).unwrap();
        self.validate_values()
    }
}

pub struct DebugOptions {
    pub is_enabled: bool,
    pub show_debug_window: bool,
    pub show_cpu_registers: bool,
    pub show_cpu_disassembler: bool,
    pub show_spc700_registers: bool,
    pub memory_map: MemoryMap,
}

impl DebugOptions {
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            show_debug_window: true,
            show_cpu_registers: true,
            show_cpu_disassembler: true,
            show_spc700_registers: true,
            memory_map: MemoryMap::new(),
        }
    }
}

pub struct ErrorMessage {
    pub show: bool,
    pub message: String,
}

impl ErrorMessage {
    pub fn new() -> Self {
        Self {
            show: false,
            message: String::from(""),
        }
    }
}

pub struct BgDebug {
    pub background: PPUBg,
    pub is_enabled: bool,
    pub texture_id: Option<imgui::TextureId>,
    pub framebuffer: Vec<u8>,
}

impl BgDebug {
    pub fn new(background: PPUBg) -> Self {
        Self {
            background: background,
            is_enabled: false,
            texture_id: None,
            framebuffer: vec![0x00; MAX_BG_WIDTH * MAX_BG_HEIGHT * 4],
        }
    }
}

pub struct PPUDebug {
    pub is_enabled: bool,
    pub show_registers: bool,
    pub show_vram: bool,
    pub backgrounds: [BgDebug; 4],
    pub vram_map: VRAMMap,
}

impl PPUDebug {
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            show_registers: true,
            show_vram: true,
            backgrounds: [
                BgDebug::new(PPUBg::Bg1),
                BgDebug::new(PPUBg::Bg2),
                BgDebug::new(PPUBg::Bg3),
                BgDebug::new(PPUBg::Bg4),
            ],
            vram_map: VRAMMap::new(),
        }
    }
}

pub struct Emulation {
    pub is_paused: bool,
}

impl Emulation {
    pub fn new() -> Self {
        Self {
            is_paused: true,
        }
    }
}

pub struct State {
    pub debug_options: DebugOptions,
    pub error_message: ErrorMessage,
    pub ppudebug: PPUDebug,
    pub emulation: Emulation,
}

impl State {
    pub fn new() -> Self {
        Self {
            debug_options: DebugOptions::new(),
            error_message: ErrorMessage::new(),
            ppudebug: PPUDebug::new(),
            emulation: Emulation::new(),
        }
    }
}
