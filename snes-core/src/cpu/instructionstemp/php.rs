use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::{CPUInstruction, push_common};
use super::decoder_common;

static INSTR_NAME: &'static str = "PHP";

pub struct PHP {}

impl CPUInstruction for PHP {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        push_common::do_push(registers, bus, &[registers.p]);
        let (bytes, cycles) = cycles::increment_cycles_php();
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
        registers.pc  = 0x0000;
        registers.p  = 0x12;
        registers.sp  = 0x1FC;
        let instruction = PHP{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(bus.read(0x1FC), 0x12);
        assert_eq!(registers.sp, 0x1FB);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.cycles, 3);
    }
}
