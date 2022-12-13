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

pub struct PPURegisters {
    data: [u8; 256],
}

impl PPURegisters {
    pub fn new() -> Self {
        Self {
            data: [0x00; 256],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[(address as usize) - 0x2100]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[(address as usize) - 0x2100] = value;
    }
}
