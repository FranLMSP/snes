use crate::cpu::cycles;
use crate::cpu::{bus::Bus, registers::Registers};
use crate::utils::addressing::AddressingMode;

use super::read_write_common::{read_8bit_from_address, write_8bit_to_address, read_16bit_from_address, write_16bit_to_address};
use super::CPUInstruction;
use super::decoder_common;

static INSTR_NAME: &str = "TRB";

pub struct TRB {
    pub addressing_mode: AddressingMode,
}

impl TRB {
    fn determine_instruction(&self, registers: &Registers) -> Box<dyn CPUInstruction> {
        match registers.is_16bit_mode() {
            true => Box::new(TRB16{addressing_mode: self.addressing_mode}),
            false => Box::new(TRB8{addressing_mode: self.addressing_mode}),
        }
    }
}

impl CPUInstruction for TRB {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let instruction = self.determine_instruction(registers);
        instruction.execute(registers, bus);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        let instruction = self.determine_instruction(registers);
        instruction.mnemonic(registers, bus, opcode)
    }
}

pub struct TRB8 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for TRB8 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let value = read_8bit_from_address(registers, bus, self.addressing_mode);
        let result = value & (!registers.a as u8);
        write_8bit_to_address(registers, bus, self.addressing_mode, result);
        registers.set_zero_flag(value & (registers.a as u8) == 0);
        let (bytes, cycles) = cycles::increment_cycles_test(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct TRB16 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for TRB16 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let value = read_16bit_from_address(registers, bus, self.addressing_mode);
        let result = value & !registers.a;
        write_16bit_to_address(registers, bus, self.addressing_mode, result);
        registers.set_zero_flag(value & registers.a == 0);
        let (bytes, cycles) = cycles::increment_cycles_test(registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

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
        registers.emulation_mode = false;
        registers.a   = 0b11001100;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.p   = 0x00;
        bus.write(0x000001, 0x03);
        bus.write(0x000002, 0x00);
        bus.write(0x000003, 0b11110011);
        registers.set_16bit_mode(false);
        let instruction = TRB8{addressing_mode: AddressingMode::Absolute};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(bus.read(0x000003), 0b00110011);
        assert_eq!(registers.a, 0b11001100);
        assert_eq!(registers.p, 0b0010_0000);
        assert_eq!(registers.pc, 0x03);
        assert_eq!(registers.cycles, 6);
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0b11001100_11001100;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.p   = 0x00;
        bus.write(0x000001, 0x03);
        bus.write(0x000002, 0x00);
        bus.write(0x000003, 0b11110000);
        bus.write(0x000004, 0b11110000);
        registers.set_16bit_mode(true);
        let instruction = TRB16{addressing_mode: AddressingMode::Absolute};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(bus.read(0x000003), 0b00110000);
        assert_eq!(bus.read(0x000004), 0b00110000);
        assert_eq!(registers.a, 0b11001100_11001100);
        assert_eq!(registers.p, 0b0000_0000);
        assert_eq!(registers.pc, 0x03);
        assert_eq!(registers.cycles, 7);
        assert!(!registers.get_zero_flag());
    }
}

