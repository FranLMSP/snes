use eframe::epaint::TextureHandle;
use snes_core::ppu::registers::{
    Background as PPUBg,
    MAX_BG_WIDTH,
    MAX_BG_HEIGHT,
};

pub struct DebugOptions {
    pub enable_debugging: bool,
    pub show_debug_options_window: bool,
    pub memory_map_conrtrol_options: MemoryMapControlOptions,
    pub cpu_debug_control_options: CPUDebugControlOptions,
    pub ppu_debug_control_options: PPUDebugControlOptions,
}

impl DebugOptions {
    pub fn new() -> Self {
        Self {
            enable_debugging: false,
            show_debug_options_window: false,
            memory_map_conrtrol_options: MemoryMapControlOptions::new(),
            cpu_debug_control_options: CPUDebugControlOptions::new(),
            ppu_debug_control_options: PPUDebugControlOptions::new(),
        }
    }
}

pub struct MemoryMapControlOptions {
    pub is_enabled: bool,
    pub inputs: MemoryMapInputs,
    pub inputs_result: MemoryMapInputs,
}

impl MemoryMapControlOptions {
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            inputs: MemoryMapInputs::new(),
            inputs_result: MemoryMapInputs::new(),
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
    pub show_upcoming_instruction: bool,
}

impl CPUDebugControlOptions {
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            show_registers: true,
            show_upcoming_instruction: true,
        }
    }
}

pub struct BgDebug {
    pub is_enabled: bool,
    pub background: PPUBg,
    pub bg_framebuffer: Vec<u8>,
    pub char_framebuffer: Vec<u8>,
    pub bg_texture: Option<TextureHandle>,
    pub char_texture: Option<TextureHandle>,
}

impl BgDebug {
    pub fn new(background: PPUBg) -> Self {
        Self {
            is_enabled: false,
            background,
            bg_framebuffer: vec![0x00; MAX_BG_WIDTH * MAX_BG_HEIGHT * 4], 
            // 8x8 pixels, 16x8 characters
            char_framebuffer: vec![0x00; 8 * 8 * 16 * 8 * 4],
            bg_texture: None,
            char_texture: None,
        }
    }
}

pub struct PPUDebugControlOptions {
    pub is_enabled: bool,
    pub show_registers: bool,
    pub show_vram: bool,
    pub show_cgram: bool,
    pub vram_inputs: VramInputs,
    pub vram_inputs_result: VramInputs,
    pub backgrounds: [BgDebug; 4],
}

impl PPUDebugControlOptions {
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            show_registers: true,
            show_vram: true,
            show_cgram: true,
            vram_inputs: VramInputs::new(),
            vram_inputs_result: VramInputs::new(),
            backgrounds: [
                BgDebug::new(PPUBg::Bg1),
                BgDebug::new(PPUBg::Bg2),
                BgDebug::new(PPUBg::Bg3),
                BgDebug::new(PPUBg::Bg4),
            ],
        }
    }
}

pub struct VramInputs {
    pub address_start: String,
    pub address_end: String,
}

impl VramInputs {
    pub fn new() -> Self {
        Self {
            address_start: String::from("0000"),
            address_end: String::from("01FF"),
        }
    }
}
