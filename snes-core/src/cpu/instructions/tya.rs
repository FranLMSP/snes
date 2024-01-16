use crate::cpu::cycles;
use crate::cpu::{bus::Bus, registers::Registers};

use super::CPUInstruction;
use super::decoder_common;

static INSTR_NAME: &str = "TYA";

pub struct TYA {}

impl TYA {
    fn determine_instruction(&self, registers: &Registers) -> Box<dyn CPUInstruction> {
        match registers.is_16bit_mode() {
            true => Box::new(TYA16{}),
            false => Box::new(TYA8{}),
        }
    }
}

impl CPUInstruction for TYA {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let instruction = self.determine_instruction(registers);
        instruction.execute(registers, bus);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        let instruction = self.determine_instruction(registers);
        instruction.mnemonic(registers, bus, opcode)
    }
}

pub struct TYA8 {}

impl CPUInstruction for TYA8 {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        let result = registers.y as u8;
        registers.set_low_a(result);
        registers.set_negative_flag((result >> 7) == 1);
        registers.set_zero_flag(result == 0);
        let (bytes, cycles) = cycles::increment_cycles_transfer();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, _registers: &Registers, _bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_single_byte_instr(opcode, INSTR_NAME)
    }
}

pub struct TYA16 {}

impl CPUInstruction for TYA16 {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        registers.a = registers.y;
        registers.set_negative_flag((registers.a >> 15) == 1);
        registers.set_zero_flag(registers.a == 0);
        let (bytes, cycles) = cycles::increment_cycles_transfer();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, _registers: &Registers, _bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_single_byte_instr(opcode, INSTR_NAME)
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
        registers.pc = 0x0000;
        registers.a = 0x0000;
        registers.y = 0xF0F0;
        registers.set_16bit_mode(false);
        let instruction = TYA8{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0x00F0);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.cycles, 2);
    }

    #[test]
    fn test_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.pc = 0x0000;
        registers.a = 0x0000;
        registers.y = 0xF0F0;
        registers.set_16bit_mode(true);
        let instruction = TYA16{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0xF0F0);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.cycles, 2);
    }
}
