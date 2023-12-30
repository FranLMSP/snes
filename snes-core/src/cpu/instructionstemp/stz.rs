use crate::cpu::cycles;
use crate::cpu::{bus::Bus, registers::Registers};
use crate::utils::addressing::AddressingMode;

use super::{CPUInstruction, Decode, write_8bit_to_address, write_16bit_to_address};
use super::decoder_common;

static INSTR_NAME: &'static str = "STZ";

pub struct STZ8 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for STZ8 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        write_8bit_to_address(registers, bus, self.addressing_mode, 0);
        let (bytes, cycles) = cycles::increment_cycles_st_index(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for STZ8 {
    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct STZ16 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for STZ16 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        write_16bit_to_address(registers, bus, self.addressing_mode, 0);
        let (bytes, cycles) = cycles::increment_cycles_st_index(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for STZ16 {
    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(true, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}


#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test_8bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.pc = 0x0000;
        registers.set_16bit_mode(false);
        bus.write(0x0002, 0x00);
        bus.write(0x0001, 0x03);
        bus.write(0x0003, 0xFF);
        let instruction = STZ8{addressing_mode: AddressingMode::Absolute};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(bus.read(0x0003), 0x00);
        assert_eq!(registers.pc, 0x0003);
        assert_eq!(registers.cycles, 4);
    }

    #[test]
    fn test_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.pc = 0x0000;
        registers.set_16bit_mode(true);
        bus.write(0x0002, 0x00);
        bus.write(0x0001, 0x03);
        bus.write(0x0003, 0xFF);
        bus.write(0x0004, 0xFF);
        let instruction = STZ16{addressing_mode: AddressingMode::Absolute};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(bus.read(0x0003), 0x00);
        assert_eq!(bus.read(0x0004), 0x00);
        assert_eq!(registers.pc, 0x0003);
        assert_eq!(registers.cycles, 4);
    }
}