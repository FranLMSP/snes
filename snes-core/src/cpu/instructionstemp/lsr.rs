use crate::{cpu::{bus::Bus, registers::Registers}, utils::{alu, addressing::AddressingMode}};

use crate::cpu::cycles;
use super::{CPUInstruction, Decode, read_8bit_from_address, write_8bit_to_address, read_16bit_from_address, write_16bit_to_address};
use super::decoder_common;

static INSTR_NAME: &'static str = "LSR";

pub struct LSR8 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for LSR8 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let (result, affected_flags) = alu::lsr(read_8bit_from_address(registers, bus, self.addressing_mode));
        write_8bit_to_address(registers, bus, self.addressing_mode, result);
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_shift(&registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for LSR8 {
    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct LSR16 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for LSR16 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let (result, affected_flags) = alu::lsr(read_16bit_from_address(registers, bus, self.addressing_mode));
        write_16bit_to_address(registers, bus, self.addressing_mode, result);
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_shift(&registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for LSR16 {
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
        registers.a   = 0b00000000_00000011;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_16bit_mode(false);
        registers.set_negative_flag(true);
        registers.set_carry_flag(false);
        let instruction = LSR8{addressing_mode: AddressingMode::Accumulator};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0b00000000_00000001);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 2);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_negative_flag());

        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0b00000001_10000011;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_16bit_mode(false);
        registers.set_negative_flag(true);
        registers.set_carry_flag(false);
        let instruction = LSR8{addressing_mode: AddressingMode::Accumulator};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0b00000001_01000001);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 2);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_negative_flag());
    }

    #[test]
    fn test_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0b00000000_00000011;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_16bit_mode(true);
        registers.set_negative_flag(true);
        registers.set_carry_flag(false);
        let instruction = LSR16{addressing_mode: AddressingMode::Accumulator};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0b00000000_00000001);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 4);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_negative_flag());

        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0b10000000_00000011;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_16bit_mode(true);
        registers.set_negative_flag(true);
        registers.set_carry_flag(true);
        let instruction = LSR16{addressing_mode: AddressingMode::Accumulator};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0b01000000_00000001);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 4);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_negative_flag());
    }
}
