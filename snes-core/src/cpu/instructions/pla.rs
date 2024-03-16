use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::{CPUInstruction, pull_common};
use super::decoder_common;

static INSTR_NAME: &str = "PLA";

pub struct PLA {}

impl CPUInstruction for PLA {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        if registers.is_16bit_mode() {
            let bytes = pull_common::do_pull(registers, bus, 2, true);
            registers.a = (bytes[0] as u16) | ((bytes[1] as u16) << 8);
        } else {
            let bytes = pull_common::do_pull(registers, bus, 1, true);
            registers.set_low_a(bytes[0]);
        }
        let (bytes, cycles) = cycles::increment_cycles_pla(registers.is_16bit_mode());
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
        registers.emulation_mode = false;
        registers.pc  = 0x0000;
        registers.y  = 0x1234;
        registers.set_16bit_mode(true);
        registers.set_negative_flag(true);
        registers.set_zero_flag(true);
        bus.write(0x1FB, 0x34);
        bus.write(0x1FC, 0x12);
        registers.sp  = 0x1FA;
        let instruction = PLA{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.a, 0x1234);
        assert_eq!(registers.sp, 0x1FC);
        assert_eq!(registers.pc, 0x0001);
        assert!(!registers.get_negative_flag());
        assert!(!registers.get_zero_flag());
        assert_eq!(registers.cycles, 5);
    }
}
