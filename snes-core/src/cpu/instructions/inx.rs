use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::{CPUInstruction, dec_common};
use super::decoder_common;

static INSTR_NAME: &str = "INX";

pub struct INX {}

impl INX {
    fn determine_instruction(&self, registers: &Registers) -> Box<dyn CPUInstruction> {
        match registers.is_16bit_index() {
            true => Box::new(INX16{}),
            false => Box::new(INX8{}),
        }
    }
}

impl CPUInstruction for INX {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let instruction = self.determine_instruction(registers);
        instruction.execute(registers, bus);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        let instruction = self.determine_instruction(registers);
        instruction.mnemonic(registers, bus, opcode)
    }
}

pub struct INX8 {}

impl CPUInstruction for INX8 {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        let result = dec_common::do_inc(
            registers,
            registers.x as u8,
        ) as u8;
        registers.set_low_x(result);
        let (bytes, cycles) = cycles::increment_cycles_inc_dec_index();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, _registers: &Registers, _bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_single_byte_instr(opcode, INSTR_NAME)
    }
}

pub struct INX16 {}

impl CPUInstruction for INX16 {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        let result = dec_common::do_inc(
            registers,
            registers.x,
        ) as u16;
        registers.x = result;
        let (bytes, cycles) = cycles::increment_cycles_inc_dec_index();
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
        registers.x   = 0x0001;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        let instruction = INX8{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.x, 2);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_negative_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.x   = 0x0001;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        let instruction = INX16{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.x, 2);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_negative_flag());
        assert!(!registers.get_zero_flag());
    }
}
