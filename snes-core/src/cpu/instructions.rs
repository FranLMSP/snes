use super::cpu::CPU;
use crate::bus::Bus;

impl CPU {
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

    pub fn adc_const(&mut self, bus: &Bus) {
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

    pub fn adc_addr(&mut self, bus: &Bus) {
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

    pub fn adc_long(&mut self, bus: &Bus) {
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

    pub fn adc_dp(&mut self, bus: &Bus) {
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

    pub fn adc_dp_indirect(&mut self, bus: &Bus) {
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

    pub fn adc_dp_indirect_long(&mut self, bus: &Bus) {
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
}

#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

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
