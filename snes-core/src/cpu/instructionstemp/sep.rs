use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use crate::utils::addressing::AddressingMode;
use super::{CPUInstruction, Decode, read_8bit_from_address};
use super::decoder_common;

static INSTR_NAME: &'static str = "SEP";

pub struct SEP {}

impl CPUInstruction for SEP {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let byte = read_8bit_from_address(registers, bus, AddressingMode::Immediate);
        registers.set_sep_byte(byte);
        let (bytes, cycles) = cycles::increment_cycles_sep();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for SEP {
    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_8bit_immediate(opcode, INSTR_NAME, registers, bus)
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
        registers.p = 0x00;
        bus.write(0x0001, 0xFF);
        let instruction = SEP{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.p, 0xFF);
        assert_eq!(registers.pc, 0x0002);
        assert_eq!(registers.cycles, 3);
    }
}
