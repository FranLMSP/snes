use super::registers::{PPURegisters, Background, MAX_TV_HEIGHT, MAX_TV_WIDTH};

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
            framebuffer: Self::initialize_tv_framebuffer(),
            registers: PPURegisters::new(),
            was_vblank_nmi_set: false,
            is_irq_set: false,
        }
    }

    fn initialize_tv_framebuffer() -> Vec<u8> {
        let mut fb = vec![0x00; FRAMEBUFFER_SIZE];
        // 0x00 also makes the alpha channel transparent, we need to make it opaque
        let mut i = 3; // start at the first alpha channel
        while i < FRAMEBUFFER_SIZE {
            fb[i] = 0xFF;
            i += 4;
        }
        fb
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
        // Objectives:
        // 1. first try to render one of the backgrounds. Background 0 for now.
        // 2. render the rest of the backgrounds
        // 3. render sprites
        // 4. figure out the priorities of each background
        self.compute_background_pixel(Background::Bg1)
        // (0xFF, 0x00, 0xFF)
    }

    // TODO: wrte tests for this function
    fn compute_background_pixel(&self, background: Background) -> (u8, u8, u8) {
        // 0. detect video mode?
        // 1. get base tileset vram address
        // 2. get base charset vram address
        // 3. know the height and width of background
        // 4. calculate which is the current tile: (x / tile width) + ((y / tile height) * background width)
        // 4.1: TODO: consider scroll values of the background for this calculation (x += x scroll, y += y scroll)
        // 5. get the tile information from vram
        // 6. get character index for the tile
        // 7. calculate the vram address of the character
        // TODO: consider that each tile can be mirrored either vertically or horizontally. Keep this in mind when fetching the character information from vram
        // 8. look up color palette
        // ----
        // possible optimizations:
        // - Fetch all of the necessary data before starting to render the scanline
        let tileset_vram_base_address = self.registers.get_bg_tile_base_address(background) as usize;
        let charset_vram_base_address = self.registers.get_bg_char_base_address(background) as usize;
        let (bg_size_width, _) = self.registers.get_bg_size(background).to_usize();
        let (tile_size_width, tile_size_height) = self.registers.get_bg_tile_size(background).to_usize();
        let vram = self.registers.vram();
        let x = (self.registers.h_count as usize) - 22; // H count ranges from 0 to 339. Pixels become visible at 22.
        let y = (self.registers.v_count as usize) - 1; // V count ranges from 0 to 261 (depending on the region). Pixels become visible at 1.

        let current_tile = (x / tile_size_width) + ((y / tile_size_height) * bg_size_width);
        let tile_byte = vram[tileset_vram_base_address + current_tile];
        let char_index = tile_byte & 0b11_11111111;
        let current_char_column = x.rem_euclid(tile_size_width);
        let current_char_row = y.rem_euclid(tile_size_height);

        let effective_vram_address =
            charset_vram_base_address +
            ((char_index as usize) * tile_size_width) +
            current_char_row;

        let vram_word = vram[effective_vram_address];
        let lsb_bitplane= vram_word as u8;
        let msb_bitplane= (vram_word >> 8) as u8;

        let pixels = Self::mix_pixel_bitplanes(lsb_bitplane, msb_bitplane);
        let effective_pixel = pixels[current_char_column];

        return match effective_pixel {
            0b00 => (0xFF, 0xFF, 0xFF),
            0b01 => (0xAC, 0xAC, 0xAC),
            0b10 => (0x56, 0x56, 0x56),
            0b11 => (0x00, 0x00, 0x00),
            _ => unreachable!(),
        }
    }

    // TODO: write tests
    fn mix_pixel_bitplanes(lsb_bitplane: u8, msb_bitplane: u8) -> [u8; 8] {
        [
            (
                (lsb_bitplane >> 7) |
                ((msb_bitplane >> 7) << 1)
            ),
            (
                ((lsb_bitplane >> 6) & 1) |
                (((msb_bitplane >> 6) & 1) << 1)
            ),
            (
                ((lsb_bitplane >> 5) & 1) |
                (((msb_bitplane >> 5) & 1) << 1)
            ),
            (
                ((lsb_bitplane >> 4) & 1) |
                (((msb_bitplane >> 4) & 1) << 1)
            ),
            (
                ((lsb_bitplane >> 3) & 1) |
                (((msb_bitplane >> 3) & 1) << 1)
            ),
            (
                ((lsb_bitplane >> 2) & 1) |
                (((msb_bitplane >> 2) & 1) << 1)
            ),
            (
                ((lsb_bitplane >> 1) & 1) |
                (((msb_bitplane >> 1) & 1) << 1)
            ),
            (
                (lsb_bitplane & 1) |
                ((msb_bitplane & 1) << 1)
            ),
        ]
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
