use super::registers::{PPURegisters, MAX_TV_HEIGHT, MAX_TV_WIDTH};

const FRAMEBUFFER_SIZE: usize = MAX_TV_HEIGHT * MAX_TV_WIDTH * 4;

pub struct PPU {
    framebuffer: Vec<u8>,
    pub registers: PPURegisters,
    was_vblank_nmi_set: bool,
    pub is_irq_set: bool,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            framebuffer: vec![0xFF; FRAMEBUFFER_SIZE],
            registers: PPURegisters::new(),
            was_vblank_nmi_set: false,
            is_irq_set: false,
        }
    }

    pub fn tick(&mut self, cpu_cycles: usize) {
        for _ in 0..(cpu_cycles * 2) {
            self.dot_cycle();
        }
    }

    pub fn dot_cycle(&mut self) {
        if !self.registers.is_vblanking() && !self.registers.is_hblanking() {
            self.put_pixel(self.compute_pixel())
        }
        self.increment_hv_count();
    }

    fn increment_hv_count(&mut self) {
        self.registers.h_count += 1;
        if self.registers.h_count > 339 {
            self.registers.h_count = 0;
            self.registers.v_count += 1;
            if self.registers.v_count > 224 && !self.was_vblank_nmi_set {
                self.registers.vblank_nmi = true;
                self.was_vblank_nmi_set = true;
            }
            if self.registers.v_count > 261 {
                self.was_vblank_nmi_set = false;
                self.registers.v_count = 0;
            }
        }
        if !self.registers.is_vblanking() {
            self.registers.vblank_nmi = false;
        }
    }

    pub fn framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }

    fn compute_pixel(&self) -> (u8, u8, u8) {
        if self.registers.is_hblanking() || self.registers.is_hblanking() {
            return (0x00, 0x00, 0x00)
        }
        (0xFF, 0x00, 0xFF)
    }

    fn put_pixel(&mut self, pixel: (u8, u8, u8)) {
        let fb_index = self.get_pixel_index();
        self.framebuffer[fb_index]     = pixel.0;
        self.framebuffer[fb_index + 1] = pixel.1;
        self.framebuffer[fb_index + 2] = pixel.2;
    }

    fn get_pixel_index(&self) -> usize {
        let h_count = self.registers.h_count as usize;
        let v_count = self.registers.v_count as usize;
        return (
            (h_count - 22) +
            ((v_count - 1) * MAX_TV_WIDTH)
        ) * 4
    }
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
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

    #[test]
    fn test_get_current_pixel_index() {
        let mut ppu = PPU::new();
        ppu.registers.v_count = 1;
        ppu.registers.h_count = 22;
        assert_eq!(ppu.get_pixel_index(), 0);
        ppu.registers.v_count = 1;
        ppu.registers.h_count = 23;
        assert_eq!(ppu.get_pixel_index(), 4);
        ppu.registers.v_count = 1;
        ppu.registers.h_count = 24;
        assert_eq!(ppu.get_pixel_index(), 8);
        ppu.registers.v_count = 2;
        ppu.registers.h_count = 22;
        assert_eq!(ppu.get_pixel_index(), 2048);
        ppu.registers.v_count = 2;
        ppu.registers.h_count = 23;
        assert_eq!(ppu.get_pixel_index(), 2052);
        ppu.registers.v_count = 2;
        ppu.registers.h_count = 24;
        assert_eq!(ppu.get_pixel_index(), 2056);
    }

    #[test]
    fn test_put_pixel() {
        let mut ppu = PPU::new();
        ppu.registers.v_count = 2;
        ppu.registers.h_count = 22;
        ppu.put_pixel((11, 22, 33));
        assert_eq!(ppu.framebuffer[2048], 11);
        assert_eq!(ppu.framebuffer[2049], 22);
        assert_eq!(ppu.framebuffer[2050], 33);
    }
}
