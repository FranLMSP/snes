use crate::bus::Bus;
use super::registers::Registers;

pub struct CPU {
    pub registers: Registers,
    pub cycles: usize,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            cycles: 0,
        }
    }

    pub fn get_absolute(&self, bus: &Bus) -> u16 {
        let pc = self.registers.get_pc_address();
        (bus.read(pc + 1) as u16) | ((bus.read(pc + 2) as u16) << 8)
    }

    pub fn get_absolute_long(&self, bus: &Bus) -> u32 {
        let pc = self.registers.get_pc_address();
        (bus.read(pc + 1) as u32) | ((bus.read(pc + 2) as u32) << 8) | ((bus.read(pc + 3) as u32) << 16)
    }

    pub fn get_direct_page(&self, bus: &Bus) -> u8 {
        let pc = self.registers.get_pc_address();
        bus.read(pc + 1)
    }

    pub fn get_immediate(&self, bus: &Bus) -> u16 {
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
mod cpu_tests {
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
}

