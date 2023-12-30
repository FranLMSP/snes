use crate::common::flags::Flags;
use crate::cpu::cycles;
use crate::cpu::{bus::Bus, registers::Registers};
use crate::utils::addressing::AddressingMode;

use super::{CPUInstruction, Decode, read_8bit_from_address, bit_common, read_16bit_from_address};
use super::decoder_common;

static INSTR_NAME: &'static str = "LDA";

pub struct LDA8 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for LDA8 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let value = read_8bit_from_address(registers, bus, self.addressing_mode);
        registers.set_flags(&[
            Flags::Negative(value >> 7 == 1),
            Flags::Zero(value == 0),
        ]);
        registers.set_low_a(value);
        bit_common::do_bit(registers, registers.a as u8, value, self.addressing_mode);
        let (bytes, cycles) = cycles::increment_cycles_lda(&registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for LDA8 {
    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(false, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
    }
}

pub struct LDA16 {
    addressing_mode: AddressingMode,
}

impl CPUInstruction for LDA16 {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let value = read_16bit_from_address(registers, bus, self.addressing_mode);
        registers.a = value;
        registers.set_flags(&[
            Flags::Negative(value >> 15 == 1),
            Flags::Zero(value == 0),
        ]);
        let (bytes, cycles) = cycles::increment_cycles_lda(&registers, self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for LDA16 {
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
        registers.a  = 0x0000;
        registers.pc  = 0x0000;
        registers.pbr  = 0x00;
        registers.set_negative_flag(false);
        registers.set_zero_flag(true);
        registers.set_16bit_mode(false);
        bus.write(0x0001, 0xFF);
        let instruction = LDA8{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pc, 0x0002);
        assert_eq!(registers.a, 0x00FF);
        assert_eq!(registers.cycles, 2);
        assert!(registers.get_negative_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.a  = 0x0000;
        registers.pc  = 0x0000;
        registers.pbr  = 0x00;
        registers.emulation_mode = false;
        registers.set_negative_flag(false);
        registers.set_zero_flag(true);
        registers.set_16bit_mode(true);
        bus.write(0x0001, 0xFF);
        bus.write(0x0002, 0xFF);
        let instruction = LDA16{addressing_mode: AddressingMode::Immediate};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pc, 0x0003);
        assert_eq!(registers.a, 0xFFFF);
        assert_eq!(registers.cycles, 3);
        assert!(registers.get_negative_flag());
        assert!(!registers.get_zero_flag());
    }
}