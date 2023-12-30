use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::{CPUInstruction, Decode, pull_common};
use super::decoder_common;

static INSTR_NAME: &'static str = "PLB";

pub struct PLB {}

impl CPUInstruction for PLB {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        registers.dbr = pull_common::do_pull(registers, bus, 1)[0];
        let (bytes, cycles) = cycles::increment_cycles_plb();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for PLB {
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
        registers.pc  = 0x0000;
        registers.dbr  = 0x00;
        registers.set_negative_flag(true);
        registers.set_zero_flag(true);
        bus.write(0x1FC, 0x12);
        registers.sp  = 0x1FB;
        let instruction = PLB{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.dbr, 0x12);
        assert_eq!(registers.sp, 0x1FC);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.get_negative_flag(), false);
        assert_eq!(registers.get_zero_flag(), false);
        assert_eq!(registers.cycles, 4);
    }
}
