use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use crate::utils::addressing::AddressingMode;
use super::read_write_common::read_8bit_from_address;
use super::CPUInstruction;
use super::decoder_common;

static INSTR_NAME: &'static str = "REP";

pub struct REP {}

impl CPUInstruction for REP {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let byte = read_8bit_from_address(registers, bus, AddressingMode::Immediate);
        registers.reset_rep_byte(byte);
        let (bytes, cycles) = cycles::increment_cycles_rep();
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

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
        registers.emulation_mode = false;
        registers.pc  = 0x0000;
        registers.p  = 0xFF;
        bus.write(0x0001, 0xFF);
        let instruction = REP{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.p, 0x00);
        assert_eq!(registers.pc, 0x0002);
        assert_eq!(registers.cycles, 3);
    }
}
