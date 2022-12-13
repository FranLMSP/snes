use super::registers::PPURegisters;

pub struct PPU {
    framebuffer: Vec<u8>,
    registers: PPURegisters,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            framebuffer: vec![],
            registers: PPURegisters,
        }
    }

    pub fn tick(&mut self, cycles: usize) {
        for _ in 0..cycles {
            self.do_cycle();
        }
    }

    pub fn do_cycle(&mut self) {

    }

    pub fn framebuffer(&self) -> &Vec<u8> {
        &self.framebuffer
    }
}