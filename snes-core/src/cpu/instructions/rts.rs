use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::{CPUInstruction, pull_common};
use super::decoder_common;

static INSTR_NAME: &str = "RTS";

pub struct RTS {}

impl CPUInstruction for RTS {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let bytes = pull_common::do_pull(registers, bus, 2, false);
        // Low byte of PC is pulled first, then high byte
        registers.pc = (bytes[0] as u16) | ((bytes[1] as u16) << 8);
        let (bytes, cycles) = cycles::increment_cycles_return_subroutine();
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
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.sp  = 0x1FA;
        bus.write(0x1FC, 0x12);
        bus.write(0x1FB, 0x34);
        let instruction = RTS{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pbr, 0x00);
        assert_eq!(registers.pc, 0x1235);
        assert_eq!(registers.cycles, 6);
    }
}
