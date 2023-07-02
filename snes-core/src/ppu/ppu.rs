use super::registers::PPURegisters;

pub struct PPU {
    framebuffer: Vec<u8>,
    h_count: u16,
    v_count: u16,
    pub registers: PPURegisters,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            framebuffer: vec![],
            h_count: 0,
            v_count: 0,
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
        self.h_count += 1;
        if self.h_count > 339 {
            self.h_count = 0;
            self.v_count += 1;
            if self.v_count > 261 {
                self.v_count = 0;
            }
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
        assert_eq!(ppu.h_count, 1);

        ppu.h_count = 339;
        ppu.increment_hv_count();
        assert_eq!(ppu.h_count, 0);
        assert_eq!(ppu.v_count, 1);

        ppu.h_count = 339;
        ppu.v_count = 261;
        ppu.increment_hv_count();
        assert_eq!(ppu.h_count, 0);
        assert_eq!(ppu.v_count, 0);
    }
}