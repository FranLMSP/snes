use crate::bus::Bus;

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
    emulation_mode: bool, // Program counter
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
            emulation_mode: false,
        }
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

    fn adc(&mut self, value: u16) {
        let carry = self.registers.get_carry_flag();
        let carry_result = match self.registers.a.checked_add(value) {
            None => true,
            Some(res) => match res.checked_add(carry as u16) {
                None => true,
                Some(_) => false,
            },
        };
        self.registers.a = self.registers.a
            .wrapping_add(value)
            .wrapping_add(carry as u16);
        self.registers.set_carry_flag(carry_result);
    }

    fn adc_const(&mut self, value: u16) {
        self.registers.pc = self.registers.pc.wrapping_add(2);
        self.cycles += 2;
        if self.registers.get_memory_select_flag() {
            self.registers.pc = self.registers.pc.wrapping_add(1);
            self.cycles += 1;
        }
        if self.registers.get_decimal_mode_flag() {
            self.cycles += 1;
        }
        self.adc(value);
    }

    fn read_immediate(&self, bus: &Bus) -> u16 {
        // If the "m" flag is set to 1, read only 8 bits.
        // Otherwise, read 16 bits
        let address = self.registers.get_pc_address();
        if self.registers.get_memory_select_flag() {
            return bus.read(address + 1) as u16;
        } else {
            return (bus.read(address + 1) as u16) | ((bus.read(address + 2) as u16) << 8);
        }

    }

    fn execute_opcode(&mut self, opcode: u8, bus: &Bus) {
        match opcode {
            0x69 => self.adc_const(self.read_immediate(bus)),
            _ => todo!("Opcode missing!"),
        }
    }

    pub fn tick(&mut self, bus: &Bus) {
        let address = self.registers.get_pc_address();
        let opcode = bus.read(address);
        self.execute_opcode(opcode, bus);
    }
}


#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test_read_immediate() {
        let mut bus = Bus::new();
        let mut cpu = CPU::new();
        cpu.registers.set_memory_select_flag(true);
        cpu.registers.pc = 0x0000;
        // write to WRAM
        bus.write(0x00_0001, 0x01);
        bus.write(0x00_0002, 0x02);
        assert_eq!(cpu.read_immediate(&bus), 0x0001);
        cpu.registers.set_memory_select_flag(false);
        assert_eq!(cpu.read_immediate(&bus), 0x0201);
    }

    #[test]
    fn test_adc() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x00;
        cpu.adc(0x40);
        assert_eq!(cpu.registers.a, 0x40);
        assert!(!cpu.registers.get_carry_flag());

        cpu.registers.a = 0xFFFF;
        cpu.adc(0x01);
        assert_eq!(cpu.registers.a, 0x0000);
        assert!(cpu.registers.get_carry_flag());

        cpu.registers.a = 0xFFFF;
        cpu.registers.set_carry_flag(true);
        cpu.adc(0x01);
        assert_eq!(cpu.registers.a, 0x0001);
        assert!(cpu.registers.get_carry_flag());

        cpu.registers.a = 0xFFFD;
        cpu.registers.set_carry_flag(true);
        cpu.adc(0x01);
        assert_eq!(cpu.registers.a, 0xFFFF);
        assert!(!cpu.registers.get_carry_flag());

        cpu.registers.a = 0x0000;
        cpu.registers.set_carry_flag(true);
        cpu.adc(0x01);
        assert_eq!(cpu.registers.a, 0x0002);
        assert!(!cpu.registers.get_carry_flag());
    }
}


#[cfg(test)]
mod cpu_registers_tests {
    use super::*;

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
        registers.p = 0x00;

        registers.set_carry_flag(true);
        assert!(registers.get_carry_flag());
        assert_eq!(registers.p, 0b0000_0001);
        registers.set_carry_flag(false);
        assert!(!registers.get_carry_flag());
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
}

