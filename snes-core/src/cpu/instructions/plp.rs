use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::{CPUInstruction, pull_common};
use super::decoder_common;

static INSTR_NAME: &str = "PLP";

pub struct PLP {}

impl CPUInstruction for PLP {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let bytes = pull_common::do_pull(registers, bus, 1, true);
        registers.p = bytes[0];
        if registers.emulation_mode {
            registers.set_memory_select_flag(true);
            registers.set_index_register_select_flag(true);
        }
        let (bytes, cycles) = cycles::increment_cycles_plp();
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
        registers.p  = 0x00;
        bus.write(0x1FC, 0xFF);
        registers.sp  = 0x1FB;
        let instruction = PLP{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.p, 0xFF);
        assert_eq!(registers.sp, 0x1FC);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.cycles, 4);
    }
}
