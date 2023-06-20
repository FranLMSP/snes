use crate::common::flags::{Flags, ModeFlag};

pub struct Registers {
    pub sp: u16, // Stack pointer
    pub x: u16, // Index X
    pub y: u16, // Index Y
    pub a: u16, // Accumulator
    pub p: u8, // Status register
    pub d: u16, // Direct page register
    pub pbr: u8, // Program bank register
    pub dbr: u8, // Data bank register
    pub pc: u16, // Program counter
    pub exposed_bit_zero: ModeFlag,
    pub emulation_mode: bool,
    pub carry: bool,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            sp: 0x01FC,
            x: 0,
            y: 0,
            a: 0,
            p: 0,
            d: 0,
            pbr: 0,
            dbr: 0,
            pc: 0,
            exposed_bit_zero: ModeFlag::EmulationMode,
            emulation_mode: true,
            carry: false,
        }
    }

    pub fn set_low_a(&mut self, val: u8) {
        self.a = (self.a & 0xFF00) | (val as u16);
    }

    pub fn set_low_x(&mut self, val: u8) {
        self.x = (self.x & 0xFF00) | (val as u16);
    }

    pub fn set_low_y(&mut self, val: u8) {
        self.y = (self.y & 0xFF00) | (val as u16);
    }

    pub fn set_low_sp(&mut self, val: u8) {
        self.sp = (self.sp & 0xFF00) | (val as u16);
    }

    pub fn get_carry_flag(&self) -> bool {
        match self.exposed_bit_zero {
            ModeFlag::Carry => self.carry,
            ModeFlag::EmulationMode => self.emulation_mode,
        }
    }

    pub fn set_carry_flag(&mut self, val: bool) {
        self.p = self.p & 0b1111_1110;
        self.p = self.p | (val as u8);
        match self.exposed_bit_zero {
            ModeFlag::Carry => self.carry = val,
            ModeFlag::EmulationMode => self.emulation_mode = val,
        };
    }

    pub fn get_emulation_mode_flag(&self) -> bool {
        self.get_carry_flag()
    }

    pub fn set_emulation_mode_flag(&mut self, val: bool) {
        self.set_carry_flag(val);
    }

    pub fn get_zero_flag(&self) -> bool {
        ((self.p & 0b0000_0010) >> 1) == 1
    }

    pub fn set_zero_flag(&mut self, val: bool) {
        self.p = self.p & 0b1111_1101;
        self.p = self.p | ((val as u8) << 1);
    }

    pub fn get_irq_disable_flag(&self) -> bool {
        ((self.p & 0b0000_0100) >> 2) == 1
    }

    pub fn set_irq_disable_flag(&mut self, val: bool) {
        self.p = self.p & 0b1111_1011;
        self.p = self.p | ((val as u8) << 2);
    }

    pub fn get_decimal_mode_flag(&self) -> bool {
        ((self.p & 0b0000_1000) >> 3) == 1
    }

    pub fn set_decimal_mode_flag(&mut self, val: bool) {
        self.p = self.p & 0b1111_0111;
        self.p = self.p | ((val as u8) << 3);
    }

    pub fn get_index_register_select_flag(&self) -> bool {
        ((self.p & 0b0001_0000) >> 4) == 1
    }

    pub fn set_index_register_select_flag(&mut self, val: bool) {
        self.p = self.p & 0b1110_1111;
        self.p = self.p | ((val as u8) << 4);
    }

    pub fn get_break_instruction_flag(&self) -> bool {
        ((self.p & 0b0001_0000) >> 4) == 1
    }

    pub fn set_break_instruction_flag(&mut self, val: bool) {
        self.p = self.p & 0b1110_1111;
        self.p = self.p | ((val as u8) << 4);
    }

    pub fn get_memory_select_flag(&self) -> bool {
        ((self.p & 0b0010_0000) >> 5) == 1
    }

    pub fn set_memory_select_flag(&mut self, val: bool) {
        self.p = self.p & 0b1101_1111;
        self.p = self.p | ((val as u8) << 5);
    }

    pub fn get_overflow_flag(&self) -> bool {
        ((self.p & 0b0100_0000) >> 6) == 1
    }

    pub fn set_overflow_flag(&mut self, val: bool) {
        self.p = self.p & 0b1011_1111;
        self.p = self.p | ((val as u8) << 6);
    }

    pub fn get_negative_flag(&self) -> bool {
        ((self.p & 0b1000_0000) >> 7) == 1
    }

    pub fn set_negative_flag(&mut self, val: bool) {
        self.p = self.p & 0b0111_1111;
        self.p = self.p | ((val as u8) << 7);
    }

    pub fn get_pc_address(&self) -> u32 {
        ((self.pbr as u32) << 16) | (self.pc as u32)
    }

    pub fn set_16bit_index(&mut self, val: bool) {
        self.set_index_register_select_flag(!val);
    }

    pub fn is_16bit_index(&self) -> bool {
        if self.emulation_mode {
            return false
        }
        !self.get_index_register_select_flag()
    }

    pub fn is_16bit_mode(&self) -> bool {
        if self.emulation_mode {
            return false
        }
        !self.get_memory_select_flag()
    }

    pub fn set_16bit_mode(&mut self, val: bool) {
        self.set_memory_select_flag(!val);
    }

    pub fn increment_pc(&mut self, bytes: u16) {
        self.pc = self.pc.wrapping_add(bytes);
    }

    pub fn decrement_pc(&mut self, bytes: u16) {
        self.pc = self.pc.wrapping_sub(bytes);
    }

    pub fn increment_sp(&mut self, bytes: u16) {
        self.sp = self.sp.wrapping_add(bytes);
    }

    pub fn decrement_sp(&mut self, bytes: u16) {
        self.sp = self.sp.wrapping_sub(bytes);
    }

    pub fn direct_page_low(&self) -> u8 {
        self.d as u8
    }

    pub fn is_emu_mode_flag_exposed(&self) -> bool {
        match self.exposed_bit_zero {
            ModeFlag::Carry => false,
            ModeFlag::EmulationMode => true,
        }
    }

    pub fn exchange_carry_and_emulation(&mut self) {
        match self.exposed_bit_zero {
            ModeFlag::Carry => {
                self.carry = self.get_carry_flag();
                self.exposed_bit_zero = ModeFlag::EmulationMode;
                self.set_emulation_mode_flag(self.emulation_mode);
            },
            ModeFlag::EmulationMode => {
                self.emulation_mode = self.get_carry_flag();
                self.exposed_bit_zero = ModeFlag::Carry;
                self.set_carry_flag(self.carry);
            },
        }
    }

    pub fn set_flags(&mut self, flags: &[Flags]) {
        for flag in flags {
            let flag = *flag;
            match flag {
                Flags::Negative(v) => self.set_negative_flag(v),
                Flags::Overflow(v) => self.set_overflow_flag(v),
                Flags::MemoryAccumulatorSelect(v) => self.set_memory_select_flag(v),
                Flags::IndexRegisterSelect(v) => self.set_index_register_select_flag(v),
                Flags::DecimalMode(v) => self.set_decimal_mode_flag(v),
                Flags::IRQDisable(v) => self.set_irq_disable_flag(v),
                Flags::Zero(v) => self.set_zero_flag(v),
                Flags::Carry(v) | Flags::EmulationMode(v) => self.set_carry_flag(v),
                Flags::HalfCarry(_) => {},
            }
        }
    }

    pub fn reset_rep_byte(&mut self, byte: u8) {
        self.p = self.p & !byte;
        // Avoid messing up exposed emu mode flag logic
        let reset_carry_flag = (byte & 0x01) == 1;
        if reset_carry_flag {
            self.set_carry_flag(false);
        }
    }
}

#[cfg(test)]
mod registers_tests {
    use super::*;

    #[test]
    fn test_is_16bit_mode() {
        let mut registers = Registers::new();
        registers.emulation_mode = false;
        registers.set_memory_select_flag(false);
        assert!(registers.is_16bit_mode());
        registers.set_memory_select_flag(true);
        assert!(!registers.is_16bit_mode());
    }

    #[test]
    fn test_set_16bit_mode() {
        let mut registers = Registers::new();
        registers.emulation_mode = false;
        registers.set_16bit_mode(true);
        assert!(registers.is_16bit_mode());
        registers.set_16bit_mode(false);
        assert!(!registers.is_16bit_mode());
    }

    #[test]
    fn test_is_16bit_index() {
        let mut registers = Registers::new();
        registers.emulation_mode = false;
        registers.set_index_register_select_flag(false);
        assert!(registers.is_16bit_index());
        registers.set_index_register_select_flag(true);
        assert!(!registers.is_16bit_index());
    }

    #[test]
    fn test_set_16bit_index() {
        let mut registers = Registers::new();
        registers.emulation_mode = false;
        registers.set_16bit_index(true);
        assert!(registers.is_16bit_index());
        registers.set_16bit_index(false);
        assert!(!registers.is_16bit_index());
    }

    #[test]
    fn test_set_low_a() {
        let mut registers = Registers::new();
        registers.emulation_mode = false;
        registers.a = 0xA1A1;
        registers.set_low_a(0xFF);
        assert_eq!(registers.a, 0xA1FF);
    }

    #[test]
    fn test_set_low_x() {
        let mut registers = Registers::new();
        registers.emulation_mode = false;
        registers.x = 0xA1A1;
        registers.set_low_x(0xFF);
        assert_eq!(registers.x, 0xA1FF);
    }

    #[test]
    fn test_set_low_y() {
        let mut registers = Registers::new();
        registers.emulation_mode = false;
        registers.y = 0xA1A1;
        registers.set_low_y(0xFF);
        assert_eq!(registers.y, 0xA1FF);
    }

    #[test]
    fn test_set_low_sp() {
        let mut registers = Registers::new();
        registers.emulation_mode = false;
        registers.sp = 0xA1A1;
        registers.set_low_sp(0xFF);
        assert_eq!(registers.sp, 0xA1FF);
    }

    #[test]
    fn test_direct_page_log() {
        let mut registers = Registers::new();
        registers.d = 0xA1A1;
        assert_eq!(registers.direct_page_low(), 0xA1);
    }

    #[test]
    fn test_get_pc_address() {
        let mut registers = Registers::new();
        registers.pc = 0x8016;
        registers.pbr = 0x01;
        assert_eq!(registers.get_pc_address(), 0x018016);
        registers.pbr = 0xFF;
        assert_eq!(registers.get_pc_address(), 0xFF8016);
        registers.pc = 0x1680;
        assert_eq!(registers.get_pc_address(), 0xFF1680);
    }

    #[test]
    fn test_status_registers() {
        let mut registers = Registers::new();
        registers.emulation_mode = false;
        registers.p = 0x00;

        registers.set_carry_flag(true);
        assert!(registers.get_carry_flag());
        assert_eq!(registers.p, 0b0000_0001);
        registers.set_carry_flag(false);
        assert!(!registers.get_carry_flag());
        assert_eq!(registers.p, 0b0000_0000);

        registers.set_emulation_mode_flag(true);
        assert!(registers.get_emulation_mode_flag());
        assert_eq!(registers.p, 0b0000_0001);
        registers.set_emulation_mode_flag(false);
        assert!(!registers.get_emulation_mode_flag());
        assert_eq!(registers.p, 0b0000_0000);

        registers.set_zero_flag(true);
        assert!(registers.get_zero_flag());
        assert_eq!(registers.p, 0b0000_0010);
        registers.set_zero_flag(false);
        assert!(!registers.get_zero_flag());
        assert_eq!(registers.p, 0b0000_0000);

        registers.set_irq_disable_flag(true);
        assert!(registers.get_irq_disable_flag());
        assert_eq!(registers.p, 0b0000_0100);
        registers.set_irq_disable_flag(false);
        assert!(!registers.get_irq_disable_flag());
        assert_eq!(registers.p, 0b0000_0000);

        registers.set_decimal_mode_flag(true);
        assert!(registers.get_decimal_mode_flag());
        assert_eq!(registers.p, 0b0000_1000);
        registers.set_decimal_mode_flag(false);
        assert!(!registers.get_decimal_mode_flag());
        assert_eq!(registers.p, 0b0000_0000);

        registers.set_memory_select_flag(true);
        assert!(registers.get_memory_select_flag());
        assert_eq!(registers.p, 0b0010_0000);
        registers.set_memory_select_flag(false);
        assert!(!registers.get_memory_select_flag());
        assert_eq!(registers.p, 0b0000_0000);

        registers.set_index_register_select_flag(true);
        assert!(registers.get_index_register_select_flag());
        assert_eq!(registers.p, 0b0001_0000);
        registers.set_index_register_select_flag(false);
        assert!(!registers.get_index_register_select_flag());
        assert_eq!(registers.p, 0b0000_0000);

        registers.set_break_instruction_flag(true);
        assert!(registers.get_break_instruction_flag());
        assert_eq!(registers.p, 0b0001_0000);
        registers.set_break_instruction_flag(false);
        assert!(!registers.get_break_instruction_flag());
        assert_eq!(registers.p, 0b0000_0000);

        registers.set_overflow_flag(true);
        assert!(registers.get_overflow_flag());
        assert_eq!(registers.p, 0b0100_0000);
        registers.set_overflow_flag(false);
        assert!(!registers.get_overflow_flag());
        assert_eq!(registers.p, 0b0000_0000);
    }

    #[test]
    fn test_set_flags() {
        let mut registers = Registers::new();
        registers.emulation_mode = false;

        registers.p = 0x00;
        registers .set_flags(&[
            Flags::Carry(true),
            Flags::Zero(true),
        ]);
        assert!(registers.get_carry_flag());
        assert!(registers.get_zero_flag());
        assert_eq!(registers.p, 0b0000_0011);

        registers.p = 0xFF;
        registers .set_flags(&[
            Flags::Carry(false),
            Flags::Zero(false),
        ]);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_zero_flag());
        assert_eq!(registers.p, 0b1111_1100);
    }

    #[test]
    fn test_exchange_carry_and_emulation() {
        let mut registers = Registers::new();
        registers.emulation_mode = false;
        registers.exposed_bit_zero = ModeFlag::Carry;
        registers .set_flags(&[Flags::Carry(false)]);
        registers.exposed_bit_zero = ModeFlag::EmulationMode;
        registers .set_flags(&[Flags::EmulationMode(false)]);

        registers.exposed_bit_zero = ModeFlag::Carry;
        registers.set_carry_flag(true);
        assert_eq!(registers.get_carry_flag(), true);
        registers.exchange_carry_and_emulation();
        assert_eq!(registers.get_carry_flag(), false);

        registers.exposed_bit_zero = ModeFlag::Carry;
        registers .set_flags(&[Flags::Carry(false)]);
        registers.exposed_bit_zero = ModeFlag::EmulationMode;
        registers .set_flags(&[Flags::EmulationMode(false)]);

        registers.exposed_bit_zero = ModeFlag::EmulationMode;
        registers.set_emulation_mode_flag(true);
        assert_eq!(registers.get_emulation_mode_flag(), true);
        registers.exchange_carry_and_emulation();
        assert_eq!(registers.get_emulation_mode_flag(), false);
    }

    #[test]
    fn test_reset_rep_byte() {
        let mut registers = Registers::new();
        registers.p = 0xFF;
        registers.reset_rep_byte(0b0000_0001);
        assert_eq!(registers.p,  0b1111_1110);

        registers.p = 0xFF;
        registers.reset_rep_byte(0b0000_0010);
        assert_eq!(registers.p,  0b1111_1101);

        registers.p = 0xFF;
        registers.reset_rep_byte(0b0010_0010);
        assert_eq!(registers.p,  0b1101_1101);

        registers.p = 0xFF;
        registers.reset_rep_byte(0b1111_1111);
        assert_eq!(registers.p,  0b0000_0000);

        registers.p = 0b0000_0010;
        registers.reset_rep_byte(0b0000_0001);
        assert_eq!(registers.p,  0b0000_0010);

        registers.p = 0b0000_0010;
        registers.reset_rep_byte(0b0000_0010);
        assert_eq!(registers.p,  0b0000_0000);

        registers.p = 0x38;
        registers.reset_rep_byte(0x38);
        assert_eq!(registers.p,  0x00);

        registers.p = 0x86;
        registers.reset_rep_byte(0x38);
        assert_eq!(registers.p,  0b10000110);
    }
}
