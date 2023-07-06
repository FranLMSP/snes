// PPU Control
pub const INIDISP: u16      = 0x2100;  // Display Control 1 (W)

pub const TM: u16           = 0x212C;  // Main Screen Designation (W)
pub const TS: u16           = 0x212D;  // Sub Screen Designation (W)

pub const SETINI: u16       = 0x2133;  // Display Control 2 (W)

// PPU BG Control
pub const BGMODE: u16       = 0x2105;  // BG Mode and BG Character Size (W)

pub const MOSAIC: u16       = 0x2106;  // Mosaic Size and Mosaic Enable (W)

pub const BG1SC: u16        = 0x2107;  // BG1 Screen Base and Screen Size (W)
pub const BG2SC: u16        = 0x2108;  // BG2 Screen Base and Screen Size (W)
pub const BG3SC: u16        = 0x2109;  // BG3 Screen Base and Screen Size (W)
pub const BG4SC: u16        = 0x210A;  // BG4 Screen Base and Screen Size (W)

pub const BG12NBA: u16      = 0x210B;  // BG Character Data Area Designation (W)
pub const BG34NBA: u16      = 0x210C;  // BG Character Data Area Designation (W)

pub const BG1HOFS: u16      = 0x210D;  // BG1 Horizontal Scroll (X) (W) and M7HOFS
pub const BG1VOFS: u16      = 0x210E;  // BG1 Vertical Scroll   (Y) (W) and M7VOFS
pub const BG2HOFS: u16      = 0x210F;  // BG2 Horizontal Scroll (X) (W)
pub const BG2VOFS: u16      = 0x2110;  // BG2 Vertical Scroll   (Y) (W)
pub const BG3HOFS: u16      = 0x2111;  // BG3 Horizontal Scroll (X) (W)
pub const BG3VOFS: u16      = 0x2112;  // BG3 Vertical Scroll   (Y) (W)
pub const BG4HOFS: u16      = 0x2113;  // BG4 Horizontal Scroll (X) (W)
pub const BG4VOFS: u16      = 0x2114;  // BG4 Horizontal Scroll (X) (W)

// PPU Rotating/Scaling
pub const M7SEL: u16        = 0x211A;  // Rotation/Scaling Mode Settings (W)

pub const M7A: u16          = 0x211B;  // Rotation/Scaling Parameter A (and Maths 16bit operand) (W)
pub const M7B: u16          = 0x211C;  // Rotation/Scaling Parameter B (and Maths 8bit operand)  (W)
pub const M7C: u16          = 0x211D;  // Rotation/Scaling Parameter C (W)
pub const M7D: u16          = 0x211E;  // Rotation/Scaling Parameter D (W)

pub const M7HOFS: u16       = 0x210D;  // BG1 Horizontal Scroll (X) (W)
pub const M7VOFS: u16       = 0x210E;  // BG1 Vertical Scroll   (Y) (W)
pub const M7X: u16          = 0x211F;  // Rotation/Scaling Center Coordinate X (W)
pub const M7Y: u16          = 0x2120;  // Rotation/Scaling Center Coordinate Y (W)

// PPU Sprites
pub const OBSEL: u16        = 0x2101;  // Object Size and Object Base (W)

// PPU Window
pub const WH0: u16          = 0x2126;  // Window 1 Left Position  (X1) (W)
pub const WH1: u16          = 0x2127;  // Window 1 Right Position (X2) (W)
pub const WH2: u16          = 0x2128;  // Window 2 Left Position  (X1) (W)
pub const WH3: u16          = 0x2129;  // Window 2 Right Position (X2) (W)

pub const W12SEL: u16       = 0x2123;  // Window BG1/BG2 Mask Settings (W)
pub const W34SEL: u16       = 0x2124;  // Window BG3/BG4 Mask Settings (W)
pub const WOBJSEL: u16      = 0x2125;  // Window OBJ/MATH Mask Settings (W)

pub const WBGLOG: u16       = 0x212A;  // Window 1 Mask Logic (W)
pub const WOBJLOG: u16      = 0x212B;  // Window 2 Mask Logic (W)

pub const TMW: u16          = 0x212E;  // Window Area Main Screen Disable (W)
pub const TSW: u16          = 0x212F;  // Window Area Sub Screen Disable (W)

// PPU Interrupts
pub const RDNMI: u16        = 0x4210;  // V-Blank NMI Flag

// PPU VRAM Access
pub const VMAINC: u16       = 0x2115; // VRAM Address Increment

pub const VMADDL: u16       = 0x2116;  // VRAM Address Low
pub const VMADDH: u16       = 0x2117;  // VRAM Address High
pub const VMDATAL: u16      = 0x2118;  // VRAM Write Low
pub const VMDATAH: u16      = 0x2119;  // VRAM Write High
pub const VMDATALW: u16     = VMDATAL;
pub const VMDATAHW: u16     = VMDATAH;
pub const RDVRAML: u16      = 0x2139;  // VRAM Read Low
pub const RDVRAMH: u16      = 0x213A;  // VRAM Read High
pub const VMDATALR: u16     = RDVRAML;
pub const VMDATAHR: u16     = RDVRAMH;


pub const MAX_BG_WIDTH: usize  = 16 * 64;
pub const MAX_BG_HEIGHT: usize = 16 * 64;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TileSize {
    P8x8,
    P16x16,
    P16x8,
}

impl TileSize {
    pub fn to_usize(&self) -> (usize, usize) {
        match self {
            Self::P8x8      => (8, 8),
            Self::P16x16    => (16, 16),
            Self::P16x8     => (16, 8),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BgSize {
    T32x32,
    T64x32, // V Mirror
    T32x64, // H Mirror
    T64x64, // H Mirror
}

impl BgSize {
    pub fn to_usize(&self) -> (usize, usize) {
        match self {
            Self::T32x32 => (32, 32),
            Self::T64x32 => (64, 32),
            Self::T32x64 => (32, 64),
            Self::T64x64 => (64, 64),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BgMode{
    Color2BPP,
    Color4BPP,
    Color8BPP,
    OffsetPerTile,
    ExtBg,
}


#[derive(Debug, Copy, Clone)]
pub enum Background {
    Bg1,
    Bg2,
    Bg3,
    Bg4,
}


pub struct PPURegisters {
    data: [u8; 256],
    vram: [u8; 0x10000],
    pub h_count: u16,
    pub v_count: u16,
}

impl PPURegisters {
    pub fn new() -> Self {
        Self {
            data: [0x00; 256],
            vram: [0; 0x10000],
            h_count: 0,
            v_count: 0,
        }
    }

    fn _read(&self, address: u16) -> u8 {
        self.data[(address as usize) - 0x2100]
    }

    pub fn _write(&mut self, address: u16, value: u8) {
        self.data[(address as usize) - 0x2100] = value;
    }

    pub fn read(&self, address: u16) -> u8 {
        self._read(address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            VMDATALW => {
                self._write(address, value);
                self.handle_write_vram(Some(value), None);
            },
            VMDATAHW => {
                self._write(address, value);
                self.handle_write_vram(None, Some(value));
            },
            VMDATALR | VMDATAHR => {},
            _ => self._write(address, value),
        };
    }

    fn handle_write_vram(&mut self, byte_lo: Option<u8>, byte_hi: Option<u8>) {
        let address = ((self.read(VMADDH) as u16) << 8) | (self.read(VMADDL) as u16);
        let effective_address = (address & 0x7FFF) * 2;
        if let Some(byte) = byte_lo {
            self.vram[effective_address.wrapping_add(1) as usize] = byte;
            self._write(VMDATALR, byte);
        }
        if let Some(byte) = byte_hi {
            self.vram[effective_address as usize] = byte;
            self._write(VMDATAHR, byte);
        }
    }

    ///  7    BG4 Tile Size (0=8x8, 1=16x16)  ;\(BgMode0..4: variable 8x8 or 16x16)
    ///  6    BG3 Tile Size (0=8x8, 1=16x16)  ; (BgMode5: 8x8 acts as 16x8)
    ///  5    BG2 Tile Size (0=8x8, 1=16x16)  ; (BgMode6: fixed 16x8?)
    ///  4    BG1 Tile Size (0=8x8, 1=16x16)  ;/(BgMode7: fixed 8x8)
    ///  3    BG3 Priority in Mode 1 (0=Normal, 1=High)
    ///  2-0  BG Screen Mode (0..7 = see below)
    pub fn get_bg_tile_size(&self, background: Background) -> TileSize {
        let byte = self.read(BGMODE);
        let bit = match background {
            Background::Bg1 => byte >> 3 & 0b1 == 1, // Bit 4
            Background::Bg2 => byte >> 4 & 0b1 == 1, // Bit 5
            Background::Bg3 => byte >> 5 & 0b1 == 1, // Bit 6
            Background::Bg4 => byte >> 6 & 0b1 == 1, // Bit 7
        };
        match bit {
            true => TileSize::P16x16,
            false => TileSize::P8x8,
        }
    }

    pub fn get_bg_size(&self, background: Background) -> BgSize {
        let byte = match background {
            Background::Bg1 => self.read(BG1SC),
            Background::Bg2 => self.read(BG2SC),
            Background::Bg3 => self.read(BG3SC),
            Background::Bg4 => self.read(BG4SC),
        };
        match byte & 0b11 {
            0 => BgSize::T32x32,
            1 => BgSize::T64x32,
            2 => BgSize::T32x64,
            3 => BgSize::T64x64,
            _ => unreachable!(),
        }
    }

    pub fn get_bg_modes(&self) -> (Option<BgMode>, Option<BgMode>, Option<BgMode>, Option<BgMode>) {
        let byte = self.read(BGMODE);
        match byte & 0b111 {
            0 => (
                Some(BgMode::Color2BPP),
                Some(BgMode::Color2BPP),
                Some(BgMode::Color2BPP),
                Some(BgMode::Color2BPP),
            ),
            1 => (
                Some(BgMode::Color4BPP),
                Some(BgMode::Color4BPP),
                Some(BgMode::Color2BPP),
                None,
            ),
            2 => (
                Some(BgMode::Color4BPP),
                Some(BgMode::Color4BPP),
                Some(BgMode::OffsetPerTile),
                None,
            ),
            3 => (
                Some(BgMode::Color8BPP),
                Some(BgMode::Color4BPP),
                None,
                None,
            ),
            4 => (
                Some(BgMode::Color8BPP),
                Some(BgMode::Color2BPP),
                Some(BgMode::OffsetPerTile),
                None,
            ),
            5 => (
                Some(BgMode::Color4BPP),
                Some(BgMode::Color2BPP),
                None,
                None,
            ),
            6 => (
                Some(BgMode::Color4BPP),
                None,
                Some(BgMode::OffsetPerTile),
                None,
            ),
            7 => (
                Some(BgMode::Color8BPP),
                Some(BgMode::ExtBg),
                None,
                None,
            ),
            _ => unreachable!(),
        }
    }

    pub fn get_bg_tile_base_address(&self, background: Background) -> u16 {
        let register = match background {
            Background::Bg1 => BG1SC,
            Background::Bg2 => BG2SC,
            Background::Bg3 => BG3SC,
            Background::Bg4 => BG4SC,
        };
        // Most significant bit is unused
        let base_address = (self.read(register) & 0b01111111) >> 2;
        let result = (base_address as u16) * 0x400;
        result
    }

    pub fn get_bg_char_base_address(&self, background: Background) -> u16 {
        let register = match background {
            Background::Bg1 => self.read(BG12NBA),
            Background::Bg2 => self.read(BG12NBA) >> 4,
            Background::Bg3 => self.read(BG34NBA),
            Background::Bg4 => self.read(BG34NBA) >> 4,
        };
        // Most significant bit is unused
        ((register as u16) & 0b111) * 0x1000
    }

    pub fn is_vblanking(&self) -> bool {
        if self.h_count >= 1 && self.h_count <= 224 {
            return false
        }
        return true
    }
}


#[cfg(test)]
mod ppu_registers_test {
    use super::*;

    #[test]
    fn test_get_bg_tile_size() {
        let mut registers = PPURegisters::new();
        registers.write(BGMODE, 0b00000100);
        assert_eq!(registers.get_bg_tile_size(Background::Bg1), TileSize::P8x8);
        registers.write(BGMODE, 0b00001100);
        assert_eq!(registers.get_bg_tile_size(Background::Bg1), TileSize::P16x16);
    }

    #[test]
    fn test_get_bg_size() {
        let mut registers = PPURegisters::new();
        registers.write(BG1SC, 2);
        assert_eq!(registers.get_bg_size(Background::Bg1), BgSize::T32x64);
    }

    #[test]
    fn test_get_bg_modes() {
        let mut registers = PPURegisters::new();
        registers.write(BGMODE, 0);
        assert_eq!(
            registers.get_bg_modes(),
            (Some(BgMode::Color2BPP), Some(BgMode::Color2BPP), Some(BgMode::Color2BPP), Some(BgMode::Color2BPP)),
        );
        registers.write(BGMODE, 1);
        assert_eq!(
            registers.get_bg_modes(),
            (Some(BgMode::Color4BPP), Some(BgMode::Color4BPP), Some(BgMode::Color2BPP), None),
        );
        registers.write(BGMODE, 2);
        assert_eq!(
            registers.get_bg_modes(),
            (Some(BgMode::Color4BPP), Some(BgMode::Color4BPP), Some(BgMode::OffsetPerTile), None),
        );
        registers.write(BGMODE, 3);
        assert_eq!(
            registers.get_bg_modes(),
            (Some(BgMode::Color8BPP), Some(BgMode::Color4BPP), None, None),
        );
        registers.write(BGMODE, 4);
        assert_eq!(
            registers.get_bg_modes(),
            (Some(BgMode::Color8BPP), Some(BgMode::Color2BPP), Some(BgMode::OffsetPerTile), None),
        );
        registers.write(BGMODE, 5);
        assert_eq!(
            registers.get_bg_modes(),
            (Some(BgMode::Color4BPP), Some(BgMode::Color2BPP), None, None),
        );
        registers.write(BGMODE, 6);
        assert_eq!(
            registers.get_bg_modes(),
            (Some(BgMode::Color4BPP), None, Some(BgMode::OffsetPerTile), None),
        );
        registers.write(BGMODE, 7);
        assert_eq!(
            registers.get_bg_modes(),
            (Some(BgMode::Color8BPP), Some(BgMode::ExtBg), None, None),
        );
    }

    #[test]
    fn test_get_bg_tile_base_address() {
        let mut registers = PPURegisters::new();
        registers.write(BG1SC, 0x00);
        assert_eq!(registers.get_bg_tile_base_address(Background::Bg1), 0x0000);
        registers.write(BG2SC, 0b00000100);
        assert_eq!(registers.get_bg_tile_base_address(Background::Bg2), 0x0400);
        registers.write(BG3SC, 0b10000100);
        assert_eq!(registers.get_bg_tile_base_address(Background::Bg3), 0x0400);
        registers.write(BG4SC, 0b11111100);
        assert_eq!(registers.get_bg_tile_base_address(Background::Bg4), 0x7C00);
        registers.write(BG1SC, 0b00001000);
        assert_eq!(registers.get_bg_tile_base_address(Background::Bg1), 0x0800);
    }

    #[test]
    fn test_get_bg_char_base_address() {
        let mut registers = PPURegisters::new();
        registers.write(BG12NBA, 0b10100001);
        registers.write(BG34NBA, 0b01011111);
        assert_eq!(registers.get_bg_char_base_address(Background::Bg1), 0x1000);
        assert_eq!(registers.get_bg_char_base_address(Background::Bg2), 0x2000);
        assert_eq!(registers.get_bg_char_base_address(Background::Bg3), 0x7000);
        assert_eq!(registers.get_bg_char_base_address(Background::Bg4), 0x5000);
    }

    #[test]
    fn test_get_vram_registers() {
        let mut registers = PPURegisters::new();
        registers.write(VMADDL, 0x00);
        registers.write(VMADDH, 0x00);
        registers.write(VMDATAHW, 0xAB);
        registers.write(VMDATALW, 0xCD);
        assert_eq!(registers.vram[0x0000], 0xAB);
        assert_eq!(registers.vram[0x0001], 0xCD);

        registers.write(VMADDH, 0x12);
        registers.write(VMADDL, 0x34);
        registers.write(VMDATAHW, 0xAB);
        registers.write(VMDATALW, 0xCD);
        assert_eq!(registers.vram[0x2468], 0xAB);
        assert_eq!(registers.vram[0x2469], 0xCD);
        assert_eq!(registers.read(VMDATAHR), 0xAB);
        assert_eq!(registers.read(VMDATALR), 0xCD);

        registers.write(VMADDH, 0xFF);
        registers.write(VMADDL, 0xFF);
        registers.write(VMDATAHW, 0xAB);
        registers.write(VMDATALW, 0xCD);
        assert_eq!(registers.vram[0xFFFE], 0xAB);
        assert_eq!(registers.vram[0xFFFF], 0xCD);
        assert_eq!(registers.read(VMDATAHR), 0xAB);
        assert_eq!(registers.read(VMDATALR), 0xCD);
    }

    #[test]
    fn test_is_vblanking() {
        let mut registers = PPURegisters::new();
        registers.h_count = 339;
        assert_eq!(registers.is_vblanking(), true);
        registers.h_count = 0;
        assert_eq!(registers.is_vblanking(), true);
        registers.h_count = 225;
        assert_eq!(registers.is_vblanking(), true);
        registers.h_count = 224;
        assert_eq!(registers.is_vblanking(), false);
        registers.h_count = 2;
        assert_eq!(registers.is_vblanking(), false);
        registers.h_count = 50;
        assert_eq!(registers.is_vblanking(), false);
    }
}
