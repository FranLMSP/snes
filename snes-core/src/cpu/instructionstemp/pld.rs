use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::{CPUInstruction, Decode, pull_common};
use super::decoder_common;

static INSTR_NAME: &'static str = "PLD";

pub struct PLD {}

impl CPUInstruction for PLD {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let bytes = pull_common::do_pull(registers, bus, 2);
        registers.d = (bytes[0] as u16) | ((bytes[1] as u16) << 8);
        let (bytes, cycles) = cycles::increment_cycles_pld();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for PLD {
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
        registers.d  = 0x1234;
        registers.set_negative_flag(true);
        registers.set_zero_flag(true);
        bus.write(0x1FB, 0x34);
        bus.write(0x1FC, 0x12);
        registers.sp  = 0x1FA;
        let instruction = PLD{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.d, 0x1234);
        assert_eq!(registers.sp, 0x1FC);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.get_negative_flag(), false);
        assert_eq!(registers.get_zero_flag(), false);
        assert_eq!(registers.cycles, 5);
    }
}
