struct Registers {
    sp: u16, // Stack pointer
    x: u16, // Index X
    y: u16, // Index Y
    a: u16, // Accumulator
    p: u8, // Status register
    d: u16, // Direct page register
    pbr: u8, // Program bank register
    dbr: u8, // Data bank register
    pc: u16, // Program counter
}

impl Registers {
    pub fn new() -> Self {
        Self {
            sp: 0,
            x: 0,
            y: 0,
            a: 0,
            p: 0,
            d: 0,
            pbr: 0,
            dbr: 0,
            pc: 0,
        }
    }

    pub fn get_p(&self) -> u8 {
        self.p
    }

    pub fn set_p(&mut self, val: u8) {
        self.p = val;
    }

    pub fn get_carry_flag(&self) -> bool {
        (self.p & 0b0000_0001) == 1
    }

    pub fn set_carry_flag(&mut self, val: bool) {
        self.p = self.p & 0b1111_1110;
        self.p = self.p | (val as u8);
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

    pub fn get_break_instruction_flag(&self) -> bool {
        ((self.p & 0b0001_0000) >> 4) == 1
    }

    pub fn set_break_instruction_flag(&mut self, val: bool) {
        self.p = self.p & 0b1110_1111;
        self.p = self.p | ((val as u8) << 4);
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
}

pub struct CPU {
    registers: Registers,
    cycles: usize,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            cycles: 0,
        }
    }

    pub fn adc_const(&mut self, value: u16) {
        self.cycles += 2;
    }
}


#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    
    #[test]
    fn test_adc_const() {
        let mut cpu = CPU::new();
        assert!(true);
    }
}


#[cfg(test)]
mod cpu_registers_tests {
    use super::*;

    
    #[test]
    fn test_status_registers() {
        let mut registers = Registers::new();
        registers.set_p(0x00);

        registers.set_carry_flag(true);
        assert!(registers.get_carry_flag());
        assert_eq!(registers.get_p(), 0b0000_0001);
        registers.set_carry_flag(false);
        assert!(!registers.get_carry_flag());
        assert_eq!(registers.get_p(), 0b0000_0000);

        registers.set_zero_flag(true);
        assert!(registers.get_zero_flag());
        assert_eq!(registers.get_p(), 0b0000_0010);
        registers.set_zero_flag(false);
        assert!(!registers.get_zero_flag());
        assert_eq!(registers.get_p(), 0b0000_0000);

        registers.set_irq_disable_flag(true);
        assert!(registers.get_irq_disable_flag());
        assert_eq!(registers.get_p(), 0b0000_0100);
        registers.set_irq_disable_flag(false);
        assert!(!registers.get_irq_disable_flag());
        assert_eq!(registers.get_p(), 0b0000_0000);

        registers.set_decimal_mode_flag(true);
        assert!(registers.get_decimal_mode_flag());
        assert_eq!(registers.get_p(), 0b0000_1000);
        registers.set_decimal_mode_flag(false);
        assert!(!registers.get_decimal_mode_flag());
        assert_eq!(registers.get_p(), 0b0000_0000);

        registers.set_break_instruction_flag(true);
        assert!(registers.get_break_instruction_flag());
        assert_eq!(registers.get_p(), 0b0001_0000);
        registers.set_break_instruction_flag(false);
        assert!(!registers.get_break_instruction_flag());
        assert_eq!(registers.get_p(), 0b0000_0000);

        registers.set_overflow_flag(true);
        assert!(registers.get_overflow_flag());
        assert_eq!(registers.get_p(), 0b0100_0000);
        registers.set_overflow_flag(false);
        assert!(!registers.get_overflow_flag());
        assert_eq!(registers.get_p(), 0b0000_0000);
    }
}

