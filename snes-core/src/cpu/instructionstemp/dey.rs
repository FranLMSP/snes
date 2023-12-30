use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::{CPUInstruction, dec_common};
use super::decoder_common;

static INSTR_NAME: &'static str = "DEY";

pub struct DEY {}

impl DEY {
    fn determine_instruction(&self, registers: &Registers) -> Box<dyn CPUInstruction> {
        match registers.is_16bit_index() {
            true => Box::new(DEY16{}),
            false => Box::new(DEY8{}),
        }
    }
}

impl CPUInstruction for DEY {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let instruction = self.determine_instruction(registers);
        instruction.execute(registers, bus);
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        let instruction = self.determine_instruction(registers);
        instruction.mnemonic(registers, bus, opcode)
    }
}

pub struct DEY8 {}

impl CPUInstruction for DEY8 {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        let result = dec_common::do_dec(
            registers,
            registers.y,
        ) as u8;
        registers.set_low_y(result);
        let (bytes, cycles) = cycles::increment_cycles_inc_dec_index();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, _registers: &Registers, _bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_single_byte_instr(opcode, INSTR_NAME)
    }
}

pub struct DEY16 {}

impl CPUInstruction for DEY16 {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        let result = dec_common::do_dec(
            registers,
            registers.y,
        ) as u16;
        registers.y = result;
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
        registers.y   = 0x0001;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        let instruction = DEY8{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.y, 0);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_negative_flag());
        assert!(registers.get_zero_flag());
    }

    #[test]
    fn test_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.y   = 0x0001;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        let instruction = DEY16{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.y, 0);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_negative_flag());
        assert!(registers.get_zero_flag());
    }
}
