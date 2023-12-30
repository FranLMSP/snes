use crate::cpu::{bus::Bus, registers::Registers};

use super::CPUInstruction;
use super::decoder_common;
use super::push_common;
use crate::cpu::cycles;

static INSTR_NAME: &'static str = "BRK";

pub struct BRK {}

impl CPUInstruction for BRK {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        push_common::do_push(registers, bus, &[registers.pbr]);
        push_common::do_push(registers, bus, &[(registers.pc >> 8) as u8, registers.pc as u8]);
        push_common::do_push(registers, bus, &[registers.p]);
        registers.set_decimal_mode_flag(false);
        registers.set_irq_disable_flag(true);
        let (bytes, cycles) = cycles::increment_cycles_brk(registers.emulation_mode);
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
    }
}
