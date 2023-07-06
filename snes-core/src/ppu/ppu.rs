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
        for _ in 0..(cpu_cycles * 2) {
            self.dot_cycle();
        }
    }

    pub fn dot_cycle(&mut self) {
        self.increment_hv_count();
    }

    fn increment_hv_count(&mut self) {
        self.registers.h_count += 1;
        if self.registers.h_count > 339 {
            self.registers.h_count = 0;
            self.registers.v_count += 1;
            if self.registers.v_count > 224 {
                self.registers.vblank_nmi = true;
            }
            if self.registers.v_count > 261 {
                self.registers.v_count = 0;
            }
        }
        if !self.registers.is_vblanking() {
            self.registers.vblank_nmi = false;
        }
    }

    pub fn framebuffer(&self) -> &Vec<u8> {
        &self.framebuffer
    }
}


#[cfg(test)]
mod ppu_general_test {
    use super::*;

    #[test]
    fn test_increment_hv_count() {
        let mut ppu = PPU::new();
        ppu.increment_hv_count();
        assert_eq!(ppu.registers.h_count, 1);

        ppu.registers.h_count = 339;
        ppu.increment_hv_count();
        assert_eq!(ppu.registers.h_count, 0);
        assert_eq!(ppu.registers.v_count, 1);

        ppu.registers.h_count = 339;
        ppu.registers.v_count = 261;
        ppu.increment_hv_count();
        assert_eq!(ppu.registers.h_count, 0);
        assert_eq!(ppu.registers.v_count, 0);
    }
}