use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::{CPUInstruction, Decode, dec_common};
use super::decoder_common;

static INSTR_NAME: &'static str = "INY";

pub struct INY8 {}

impl CPUInstruction for INY8 {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        let result = dec_common::do_inc(
            registers,
            registers.y,
        ) as u8;
        registers.set_low_y(result);
        let (bytes, cycles) = cycles::increment_cycles_inc_dec_index();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for INY8 {
    fn mnemonic(&self, _registers: &Registers, _bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_single_byte_instr(opcode, INSTR_NAME)
    }
}

pub struct INY16 {}

impl CPUInstruction for INY16 {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        let result = dec_common::do_inc(
            registers,
            registers.y,
        ) as u16;
        registers.y = result;
        let (bytes, cycles) = cycles::increment_cycles_inc_dec_index();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for INY16 {
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
        let instruction = INY8{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.y, 2);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_negative_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.y   = 0x0001;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        let instruction = INY16{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.y, 2);
        assert_eq!(registers.pc, 0x01);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_negative_flag());
        assert!(!registers.get_zero_flag());
    }
}
