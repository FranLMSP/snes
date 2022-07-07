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

    fn adc8bcd(&mut self, value: u8) {
        let carry = self.registers.get_carry_flag();
        let a = self.registers.a as u8;
        let carry_result = match a.checked_add(value) {
            None => true,
            Some(res) => match res.checked_add(carry as u8) {
                None => true,
                Some(_) => false,
            },
        };
        let result = a
            .wrapping_add(value)
            .wrapping_add(carry as u8);
        self.registers.a = (self.registers.a & 0xFF00) | (result as u16);
        self.registers.set_carry_flag(carry_result);
        self.registers.set_negative_flag((result >> 7) == 1);
        self.registers.set_zero_flag(result == 0);
    }

    fn adc16bcd(&mut self, value: u16) {
        let carry = self.registers.get_carry_flag();
        let carry_result = match self.registers.a.checked_add(value) {
            None => true,
            Some(res) => match res.checked_add(carry as u16) {
                None => true,
                Some(_) => false,
            },
        };
        let result = self.registers.a
            .wrapping_add(value)
            .wrapping_add(carry as u16);
        self.registers.a = result;
        self.registers.set_carry_flag(carry_result);
        self.registers.set_negative_flag((result >> 7) == 1);
        self.registers.set_zero_flag(result == 0);
    }

    fn adc8(&mut self, value: u8) {
        let carry = self.registers.get_carry_flag();
        let a = self.registers.a as u8;
        let carry_result = match a.checked_add(value) {
            None => true,
            Some(res) => match res.checked_add(carry as u8) {
                None => true,
                Some(_) => false,
            },
        };
        let result = a
            .wrapping_add(value)
            .wrapping_add(carry as u8);
        self.registers.a = (self.registers.a & 0xFF00) | (result as u16);
        self.registers.set_carry_flag(carry_result);
        self.registers.set_negative_flag((result >> 7) == 1);
        self.registers.set_zero_flag(result == 0);
    }

    fn adc16(&mut self, value: u16) {
        let carry = self.registers.get_carry_flag();
        let carry_result = match self.registers.a.checked_add(value) {
            None => true,
            Some(res) => match res.checked_add(carry as u16) {
                None => true,
                Some(_) => false,
            },
        };
        let result = self.registers.a
            .wrapping_add(value)
            .wrapping_add(carry as u16);
        self.registers.a = result;
        self.registers.set_carry_flag(carry_result);
        self.registers.set_negative_flag((result >> 15) == 1);
        self.registers.set_zero_flag(result == 0);
    }

    fn adc(&mut self, value: u16) {
        // if the M flag is set, perform 8 bit addition.
        // Otherwise, 16 bit addition
        if self.registers.get_memory_select_flag() {
            if self.registers.get_decimal_mode_flag() {
                self.adc8bcd(value as u8);
            } else {
                self.adc8(value as u8);
            }
        } else {
            if self.registers.get_decimal_mode_flag() {
                self.adc16bcd(value);
            } else {
                self.adc16(value);
            }
        }
    }

    fn adc_const(&mut self, bus: &Bus) {
        let value = self.get_immediate(bus);
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

    fn adc_addr(&mut self, bus: &Bus) {
        let address = ((self.registers.pbr as u32) << 16) | (self.get_absolute(bus) as u32);
        self.registers.pc = self.registers.pc.wrapping_add(3);
        self.cycles += 4;
        if self.registers.get_memory_select_flag() {
            self.cycles += 1;
        }
        if self.registers.get_decimal_mode_flag() {
            self.cycles += 1;
        }
        let value = bus.read(address);
        self.adc(value as u16);
    }

    fn adc_long(&mut self, bus: &Bus) {
        let address = self.get_absolute_long(bus);
        self.registers.pc = self.registers.pc.wrapping_add(4);
        self.cycles += 5;
        if self.registers.get_memory_select_flag() {
            self.cycles += 1;
        }
        if self.registers.get_decimal_mode_flag() {
            self.cycles += 1;
        }
        let value = bus.read(address);
        self.adc(value as u16);
    }

    fn adc_dp(&mut self, bus: &Bus) {
        let address = self.get_direct_page(bus);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        self.cycles += 3;
        if self.registers.get_memory_select_flag() {
            self.cycles += 1;
        }
        if address != 0 {
            self.cycles += 1;
        }
        if self.registers.get_decimal_mode_flag() {
            self.cycles += 1;
        }
        let value = bus.read(address as u32);
        self.adc(value as u16);
    }

    fn adc_dp_indirect(&mut self, bus: &Bus) {
        let pointer = self.get_direct_page(bus) as u32;
        let address = ((bus.read(pointer) as u32) << 8) | (bus.read(pointer + 1) as u32);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        self.cycles += 3;
        if self.registers.get_memory_select_flag() {
            self.cycles += 1;
        }
        if address != 0 {
            self.cycles += 1;
        }
        if self.registers.get_decimal_mode_flag() {
            self.cycles += 1;
        }
        let value = bus.read(address);
        self.adc(value as u16);
    }

    fn adc_dp_indirect_long(&mut self, bus: &Bus) {
        let pointer = self.get_direct_page(bus) as u32;
        let address = ((bus.read(pointer) as u32) << 16) | ((bus.read(pointer + 1) as u32) << 8) | (bus.read(pointer + 2) as u32);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        self.cycles += 3;
        if self.registers.get_memory_select_flag() {
            self.cycles += 1;
        }
        if address != 0 {
            self.cycles += 1;
        }
        if self.registers.get_decimal_mode_flag() {
            self.cycles += 1;
        }
        let value = bus.read(address);
        self.adc(value as u16);
    }

    fn get_absolute(&self, bus: &Bus) -> u16 {
        let pc = self.registers.get_pc_address();
        (bus.read(pc + 1) as u16) | ((bus.read(pc + 2) as u16) << 8)
    }

    fn get_absolute_long(&self, bus: &Bus) -> u32 {
        let pc = self.registers.get_pc_address();
        (bus.read(pc + 1) as u32) | ((bus.read(pc + 2) as u32) << 8) | ((bus.read(pc + 3) as u32) << 16)
    }

    fn get_direct_page(&self, bus: &Bus) -> u8 {
        let pc = self.registers.get_pc_address();
        bus.read(pc + 1)
    }

    fn get_immediate(&self, bus: &Bus) -> u16 {
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
            0x69 => self.adc_const(bus),
            0x6D => self.adc_addr(bus),
            0x6F => self.adc_long(bus),
            0x65 => self.adc_dp(bus),
            0x72 => self.adc_dp_indirect(bus),
            0x67 => self.adc_dp_indirect_long(bus),
            _ => todo!("Missing opcode implementation: {:02X}", opcode),
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
    fn test_get_immediate() {
        let mut bus = Bus::new();
        let mut cpu = CPU::new();
        cpu.registers.set_memory_select_flag(true);
        cpu.registers.pc = 0x0000;
        cpu.registers.pbr = 0x00;
        // write to WRAM
        bus.write(0x00_0001, 0x01);
        bus.write(0x00_0002, 0x02);
        assert_eq!(cpu.get_immediate(&bus), 0x0001);
        cpu.registers.set_memory_select_flag(false);
        assert_eq!(cpu.get_immediate(&bus), 0x0201);

        cpu.registers.set_memory_select_flag(true);
        cpu.registers.pc = 0x0010;
        cpu.registers.pbr = 0x7E;
        // write to WRAM
        bus.write(0x7E_0011, 0x01);
        bus.write(0x7E_0012, 0x02);
        assert_eq!(cpu.get_immediate(&bus), 0x0001);
        cpu.registers.set_memory_select_flag(false);
        assert_eq!(cpu.get_immediate(&bus), 0x0201);
    }

    #[test]
    fn test_get_absolute() {
        let mut bus = Bus::new();
        let mut cpu = CPU::new();
        cpu.registers.pc = 0x0000;
        cpu.registers.pbr = 0x00;
        // write to WRAM
        bus.write(0x00_0001, 0x01);
        bus.write(0x00_0002, 0x02);
        assert_eq!(cpu.get_absolute(&bus), 0x0201);

        cpu.registers.pc = 0x0010;
        cpu.registers.pbr = 0x7E;
        // write to WRAM
        bus.write(0x7E_0011, 0x01);
        bus.write(0x7E_0012, 0x02);
        assert_eq!(cpu.get_absolute(&bus), 0x0201);
    }

    #[test]
    fn test_get_absolute_long() {
        let mut bus = Bus::new();
        let mut cpu = CPU::new();
        cpu.registers.pc = 0x0000;
        cpu.registers.pbr = 0x00;
        // write to WRAM
        bus.write(0x00_0001, 0x01);
        bus.write(0x00_0002, 0x02);
        bus.write(0x00_0003, 0x03);
        assert_eq!(cpu.get_absolute_long(&bus), 0x030201);

        cpu.registers.pc = 0x0010;
        cpu.registers.pbr = 0x7E;
        // write to WRAM
        bus.write(0x7E_0011, 0x01);
        bus.write(0x7E_0012, 0x02);
        bus.write(0x7E_0013, 0x03);
        assert_eq!(cpu.get_absolute_long(&bus), 0x030201);
    }

    #[test]
    fn test_get_direct_page() {
        let mut bus = Bus::new();
        let mut cpu = CPU::new();
        cpu.registers.pc = 0x0000;
        cpu.registers.pbr = 0x00;
        // write to WRAM
        bus.write(0x00_0001, 0x01);
        assert_eq!(cpu.get_direct_page(&bus), 0x01);

        cpu.registers.pc = 0x0010;
        cpu.registers.pbr = 0x7E;
        // write to WRAM
        bus.write(0x7E_0011, 0x01);
        assert_eq!(cpu.get_direct_page(&bus), 0x01);
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

