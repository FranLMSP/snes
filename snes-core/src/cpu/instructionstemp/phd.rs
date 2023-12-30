use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::{CPUInstruction, Decode, push_common};
use super::decoder_common;

static INSTR_NAME: &'static str = "PHD";

pub struct PHD {}

impl CPUInstruction for PHD {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let value = registers.d;
        push_common::do_push(registers, bus, &[(value >> 8) as u8, value as u8]);
        let (bytes, cycles) = cycles::increment_cycles_phd();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for PHD {
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
        registers.sp  = 0x1FC;
        registers.d = 0x1234;
        let instruction = PHD{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(bus.read(0x1FC), 0x12);
        assert_eq!(bus.read(0x1FB), 0x34);
        assert_eq!(registers.sp, 0x1FA);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.cycles, 4);
    }
}
