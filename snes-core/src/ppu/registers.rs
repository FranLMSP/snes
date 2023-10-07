// PPU Control
pub const INIDISP: u16      = 0x2100;  // Display Control 1 (W)

// PPU Sprites
pub const OBSEL: u16        = 0x2101;  // Object Size and Object Base (W)

// OAM Read/Write
pub const OAMADDL: u16      = 0x2102;  // OAM Address (L)
pub const OAMADDH: u16      = 0x2103;  // OAM Address (H)
pub const OAMDATA: u16      = 0x2104;  // OAM Data Write

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

// PPU VRAM Access
pub const VMAIN: u16        = 0x2115; // VRAM Address Increment
pub const VMADDL: u16       = 0x2116;  // VRAM Address Low
pub const VMADDH: u16       = 0x2117;  // VRAM Address High
pub const VMDATAL: u16      = 0x2118;  // VRAM Write Low
pub const VMDATAH: u16      = 0x2119;  // VRAM Write High

// PPU Rotating/Scaling
pub const M7SEL: u16        = 0x211A;  // Rotation/Scaling Mode Settings (W)
pub const M7A: u16          = 0x211B;  // Rotation/Scaling Parameter A (and Maths 16bit operand) (W)
pub const M7B: u16          = 0x211C;  // Rotation/Scaling Parameter B (and Maths 8bit operand)  (W)
pub const M7C: u16          = 0x211D;  // Rotation/Scaling Parameter C (W)
pub const M7D: u16          = 0x211E;  // Rotation/Scaling Parameter D (W)
pub const M7X: u16          = 0x211F;  // Rotation/Scaling Center Coordinate X (W)
pub const M7Y: u16          = 0x2120;  // Rotation/Scaling Center Coordinate Y (W)

// PPU CGRAM
pub const CGADD: u16        = 0x2121;  // Palette CGRAM Address
pub const CGDATA: u16       = 0x2122;  // Palette CGRAM Address

// PPU Window
pub const W12SEL: u16       = 0x2123;  // Window BG1/BG2 Mask Settings (W)
pub const W34SEL: u16       = 0x2124;  // Window BG3/BG4 Mask Settings (W)
pub const WOBJSEL: u16      = 0x2125;  // Window OBJ/MATH Mask Settings (W)
pub const WH0: u16          = 0x2126;  // Window 1 Left Position  (X1) (W)
pub const WH1: u16          = 0x2127;  // Window 1 Right Position (X2) (W)
pub const WH2: u16          = 0x2128;  // Window 2 Left Position  (X1) (W)
pub const WH3: u16          = 0x2129;  // Window 2 Right Position (X2) (W)
pub const WBGLOG: u16       = 0x212A;  // Window 1 Mask Logic (W)
pub const WOBJLOG: u16      = 0x212B;  // Window 2 Mask Logic (W)
pub const TM: u16           = 0x212C;  // Main Screen Designation (W)
pub const TS: u16           = 0x212D;  // Sub Screen Designation (W)
pub const TMW: u16          = 0x212E;  // Window Area Main Screen Disable (W)
pub const TSW: u16          = 0x212F;  // Window Area Sub Screen Disable (W)

// PPU Color Math
pub const CGWSEL: u16       = 0x2130;  // Color Math Control Register A
pub const CGADSUB: u16      = 0x2131;  // Color Math Control Register B
pub const COLDATA: u16      = 0x2132;  // Color Math Sub Screen Backdrop Color

// Display Control
pub const SETINI: u16       = 0x2133;  // Display Control 2 (W)

// PPU Read-only ports
pub const MPYL: u16         = 0x2134;  // Signed Multiply Result (lower 8bit)
pub const MPYM: u16         = 0x2135;  // Signed Multiply Result (middle 8bit)
pub const MPYH: u16         = 0x2136;  // Signed Multiply Result (upper 8bit)
pub const SLHV: u16         = 0x2137;  // Latch H/V-Counter by Software (Read=Strobe)
pub const RDOAM: u16        = 0x2138;  // OAM Data Read            (read-twice)
pub const RDVRAML: u16      = 0x2139;  // VRAM Read Low
pub const RDVRAMH: u16      = 0x213A;  // VRAM Read High
pub const RDCGRAM: u16      = 0x213B;  // CGRAM Data Read (Palette)(read-twice)
pub const OPHCT: u16        = 0x213C;  // Horizontal Counter Latch (read-twice)
pub const OPVCT: u16        = 0x213D;  // Vertical Counter Latch   (read-twice)
pub const STAT77: u16       = 0x213E;  // PPU1 Status and PPU1 Version Number
pub const STAT78: u16       = 0x213F;  // PPU1 Status and PPU1 Version Number


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
    data: [u8; 64],
    vram: [u16; 0x8000],
    pub vblank_nmi: bool,
    pub h_count: u16,
    pub v_count: u16,
}

impl PPURegisters {
    pub fn new() -> Self {
        Self {
            data: [0x00; 64],
            vram: [0; 0x8000],
            vblank_nmi: false,
            h_count: 0,
            v_count: 0,
        }
    }

    pub fn registers(&self) -> &[u8] {
        &self.data
    }

    pub fn vram(&self) -> &[u16] {
        &self.vram
    }

    fn _read(&self, address: u16) -> u8 {
        match address {
            0x2100..=0x213F => self.data[(address as usize) - 0x2100],
            _ => 0xFF,
        }
    }

    pub fn _write(&mut self, address: u16, value: u8) {
        match address {
            0x2100..=0x213F => self.data[(address as usize) - 0x2100] = value,
            _ => {},
        };
    }

    pub fn read_external(&self, address: u16) -> u8 {
        self._read(address)
    }

    pub fn read(&mut self, address: u16) -> u8 {
        let result = self._read(address);
        match address {
            VMDATAL => self.handle_vram_addr_auto_increment(Some(result), None),
            VMDATAH => self.handle_vram_addr_auto_increment(None, Some(result)),
            _ => {},
        };
        self._read(address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            VMDATAL => {
                self._write(address, value);
                self.handle_write_vram(Some(value), None);
            },
            VMDATAH => {
                self._write(address, value);
                self.handle_write_vram(None, Some(value));
            },
            RDVRAML | RDVRAMH => {},
            _ => self._write(address, value),
        };
    }

    fn handle_write_vram(&mut self, byte_lo: Option<u8>, byte_hi: Option<u8>) {
        let address = (((self.read(VMADDH) as u16) << 8) | (self.read(VMADDL) as u16)) & 0x7FFF;
        let current_word = self.vram[address as usize];
        if let Some(byte) = byte_lo {
            self.vram[address as usize] = (current_word & 0xFF00) | (byte as u16);
            self._write(RDVRAML, byte);
        }
        if let Some(byte) = byte_hi {
            self.vram[address as usize] = (current_word & 0x00FF) | ((byte as u16) << 8);
            self._write(RDVRAMH, byte);
        }
        self.handle_vram_addr_auto_increment(byte_lo, byte_hi);
    }

    fn handle_vram_addr_auto_increment(&mut self, byte_lo: Option<u8>, byte_hi: Option<u8>) {
        let register = self._read(VMAIN);
        let amount_to_increment = match register & 0b11 {
            0b00 => 1,
            0b01 => 32,
            0b10 => 128,
            0b11 => 128,
            _ => unreachable!(),
        };
        let increment_when_lo = (register >> 7) != 1;
        let increment_when_hi = !increment_when_lo;
        let current_value = self.get_current_vram_address();
        if increment_when_lo {
            if let Some(_) = byte_lo {
                self.set_current_vram_address(current_value.wrapping_add(amount_to_increment));
            }
        }
        if increment_when_hi {
            if let Some(_) = byte_hi {
                self.set_current_vram_address(current_value.wrapping_add(amount_to_increment));
            }
        }
    }

    fn get_current_vram_address(&self) -> u16 {
        ((self._read(VMADDH) as u16) << 8) | (self._read(VMADDL) as u16)
    }

    fn set_current_vram_address(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self._write(VMADDH, bytes[0]);
        self._write(VMADDL, bytes[1]);
    }

    ///  7    BG4 Tile Size (0=8x8, 1=16x16)  ;\(BgMode0..4: variable 8x8 or 16x16)
    ///  6    BG3 Tile Size (0=8x8, 1=16x16)  ; (BgMode5: 8x8 acts as 16x8)
    ///  5    BG2 Tile Size (0=8x8, 1=16x16)  ; (BgMode6: fixed 16x8?)
    ///  4    BG1 Tile Size (0=8x8, 1=16x16)  ;/(BgMode7: fixed 8x8)
    ///  3    BG3 Priority in Mode 1 (0=Normal, 1=High)
    ///  2-0  BG Screen Mode (0..7 = see below)
    pub fn get_bg_tile_size(&self, background: Background) -> TileSize {
        let byte = self._read(BGMODE);
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
            Background::Bg1 => self._read(BG1SC),
            Background::Bg2 => self._read(BG2SC),
            Background::Bg3 => self._read(BG3SC),
            Background::Bg4 => self._read(BG4SC),
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
        let byte = self._read(BGMODE);
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
        let base_address = (self._read(register) & 0b01111111) >> 2;
        let result = (base_address as u16) * 0x400;
        result
    }

    pub fn get_bg_char_base_address(&self, background: Background) -> u16 {
        let register = match background {
            Background::Bg1 => self._read(BG12NBA),
            Background::Bg2 => self._read(BG12NBA) >> 4,
            Background::Bg3 => self._read(BG34NBA),
            Background::Bg4 => self._read(BG34NBA) >> 4,
        };
        // Most significant bit is unused
        ((register as u16) & 0b111) * 0x1000
    }

    pub fn is_vblanking(&self) -> bool {
        if self.v_count >= 1 && self.v_count <= 224 {
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
        registers.write(VMDATAH, 0xAB);
        registers.write(VMDATAL, 0xCD);
        assert_eq!(registers.vram[0x0000], 0xABCD);

        registers.write(VMADDH, 0x12);
        registers.write(VMADDL, 0x34);
        registers.write(VMDATAH, 0xAB);
        registers.write(VMDATAL, 0xCD);
        assert_eq!(registers.vram[0x1234], 0xABCD);
        assert_eq!(registers.read(RDVRAMH), 0xAB);
        assert_eq!(registers.read(RDVRAML), 0xCD);

        registers.write(VMADDH, 0x7F);
        registers.write(VMADDL, 0xFF);
        registers.write(VMDATAH, 0xAB);
        registers.write(VMDATAL, 0xCD);
        assert_eq!(registers.vram[0x7FFF], 0xABCD);
        assert_eq!(registers.read(RDVRAMH), 0xAB);
        assert_eq!(registers.read(RDVRAML), 0xCD);
    }

    #[test]
    fn test_auto_increment_vram_address() {
        let mut registers = PPURegisters::new();
        // Increment after writing
        registers.write(VMADDL, 0x00);
        registers.write(VMADDH, 0x00);

        // Increment when low bit is written to
        registers.write(VMAIN, 0x00);

        registers.write(VMDATAH, 0xAA);
        assert_eq!(registers.get_current_vram_address(), 0x0000);
        registers.write(VMDATAL, 0xAA);
        assert_eq!(registers.get_current_vram_address(), 0x0001);

        // Increment when hi bit is written to
        registers.write(VMAIN, 0b1000_0000);
        registers.write(VMDATAH, 0xAA);
        assert_eq!(registers.get_current_vram_address(), 0x0002);
        registers.write(VMDATAL, 0xAA);
        assert_eq!(registers.get_current_vram_address(), 0x0002);


        // Increment after reading
        registers.write(VMADDL, 0x00);
        registers.write(VMADDH, 0x00);

        // Increment when low bit is read from
        registers.write(VMAIN, 0x00);

        registers.read(VMDATAH);
        assert_eq!(registers.get_current_vram_address(), 0x0000);
        registers.read(VMDATAL);
        assert_eq!(registers.get_current_vram_address(), 0x0001);

        // Increment when hi bit is read from
        registers.write(VMAIN, 0b1000_0000);
        registers.read(VMDATAH);
        assert_eq!(registers.get_current_vram_address(), 0x0002);
        registers.read(VMDATAL);
        assert_eq!(registers.get_current_vram_address(), 0x0002);

        // Increment amounts
        registers.write(VMAIN, 0b1000_0000);
        registers.write(VMADDL, 0x00);
        registers.write(VMADDH, 0x00);
        registers.write(VMDATAH, 0xAA);
        assert_eq!(registers.get_current_vram_address(), 1);

        registers.write(VMAIN, 0b1000_0001);
        registers.write(VMADDL, 0x00);
        registers.write(VMADDH, 0x00);
        registers.write(VMDATAH, 0xAA);
        assert_eq!(registers.get_current_vram_address(), 32);

        registers.write(VMAIN, 0b1000_0010);
        registers.write(VMADDL, 0x00);
        registers.write(VMADDH, 0x00);
        registers.write(VMDATAH, 0xAA);
        assert_eq!(registers.get_current_vram_address(), 128);

        registers.write(VMAIN, 0b1000_0011);
        registers.write(VMADDL, 0x00);
        registers.write(VMADDH, 0x00);
        registers.write(VMDATAH, 0xAA);
        assert_eq!(registers.get_current_vram_address(), 128);
    }

    #[test]
    fn test_is_vblanking() {
        let mut registers = PPURegisters::new();
        registers.v_count = 339;
        assert_eq!(registers.is_vblanking(), true);
        registers.v_count = 0;
        assert_eq!(registers.is_vblanking(), true);
        registers.v_count = 225;
        assert_eq!(registers.is_vblanking(), true);
        registers.v_count = 224;
        assert_eq!(registers.is_vblanking(), false);
        registers.v_count = 2;
        assert_eq!(registers.is_vblanking(), false);
        registers.v_count = 50;
        assert_eq!(registers.is_vblanking(), false);
    }
}
