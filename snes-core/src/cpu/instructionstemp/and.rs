use crate::{cpu::{bus::Bus, registers::Registers}, utils::{alu, addressing::AddressingMode}};

use crate::cpu::cycles;
use super::{CPUInstruction, Decode, read_8bit_from_address, read_16bit_from_address};
use super::decoder_common;

static INSTR_NAME: &'static str = "AND";

pub struct AND8 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for AND8 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let (result, affected_flags) = alu::and(
            registers.a as u8,
            read_8bit_from_address(registers, bus, self.addressing_mode),
        );
        registers.set_low_a(result);
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_bitwise(&registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for AND8 {
    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct AND16 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for AND16 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let (result, affected_flags) = alu::and(
            registers.a,
            read_16bit_from_address(registers, bus, self.addressing_mode),
        );
        registers.a = result;
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_bitwise(&registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for AND16 {
    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(true, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}


#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test_and_8bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0x0101;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(true);
        bus.write(0x000001, 0x00);
        let instruction = AND8{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0x0100);
        assert_eq!(registers.pc, 0x02);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_carry_flag());
        assert!(registers.get_zero_flag());
    }

    #[test]
    fn test_and_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0x0101;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(false);
        bus.write(0x000001, 0x01);
        bus.write(0x000002, 0x01);
        let instruction = AND16{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0x0101);
        assert_eq!(registers.pc, 0x03);
        assert_eq!(registers.cycles, 3);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_zero_flag());
    }
}
