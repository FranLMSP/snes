use super::registers::PPURegisters;

pub struct PPU {
    framebuffer: Vec<u8>,
    pub registers: PPURegisters,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            framebuffer: vec![],
            registers: PPURegisters::new(),
        }
    }

    pub fn tick(&mut self, cpu_cycles: usize) {
        for _ in 0..cpu_cycles {
            self.do_cycle();
        }
    }

    pub fn do_cycle(&mut self) {

    }

    pub fn framebuffer(&self) -> &Vec<u8> {
        &self.framebuffer
    }
}