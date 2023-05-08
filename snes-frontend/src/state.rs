extern crate snes_core;
use snes_core::ppu::registers::{
    Background as PPUBg,
    MAX_BG_WIDTH,
    MAX_BG_HEIGHT,
};


pub struct DebugOptions {
    pub is_enabled: bool,
    pub show_debug_window: bool,
    pub show_cpu_registers: bool,
    pub show_spc700_registers: bool,
    pub show_cpu_memory: bool,
}

impl DebugOptions {
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            show_debug_window: true,
            show_cpu_registers: true,
            show_spc700_registers: true,
            show_cpu_memory: true,
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
    pub backgrounds: [BgDebug; 4],
}

impl PPUDebug {
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            backgrounds: [
                BgDebug::new(PPUBg::Bg1),
                BgDebug::new(PPUBg::Bg2),
                BgDebug::new(PPUBg::Bg3),
                BgDebug::new(PPUBg::Bg4),
            ],
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