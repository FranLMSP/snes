use crate::cpu::cycles;
use crate::cpu::{bus::Bus, registers::Registers};

use super::CPUInstruction;
use super::decoder_common;

static INSTR_NAME: &'static str = "TCS";

pub struct TCS {}

impl CPUInstruction for TCS {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        let result = registers.a;
        registers.sp = result;
        registers.set_negative_flag((result >> 7) == 1);
        registers.set_zero_flag(result == 0);
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
    fn test() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.pc = 0x0000;
        registers.a = 0xF0F0;
        registers.sp = 0x0000;
        let instruction = TCS{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.sp, 0xF0F0);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.cycles, 2);
    }
}
