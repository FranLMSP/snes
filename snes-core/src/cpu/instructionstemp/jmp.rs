use crate::cpu::{bus::Bus, registers::Registers};
use crate::utils::addressing::AddressingMode;

use super::read_write_common::get_effective_address;
use super::CPUInstruction;
use super::decoder_common;
use crate::cpu::cycles;

static INSTR_NAME: &'static str = "JMP";

pub struct JMP {
    pub addressing_mode: AddressingMode,
}

impl CPUInstruction for JMP {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let effective_address = get_effective_address(&registers, bus, self.addressing_mode);
        let is_long = match self.addressing_mode {
            AddressingMode::AbsoluteLong |
            AddressingMode::AbsoluteIndirectLong => true,
            _  => false,
        };
        registers.pc = effective_address as u16;
        if is_long {
            registers.pbr = (effective_address >> 16) as u8;
        }
        let (bytes, cycles) = cycles::increment_cycles_jmp(self.addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }

    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String {
        decoder_common::mnemonic_arithmetic(true, opcode, INSTR_NAME, self.addressing_mode, registers, bus)
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
        bus.write(0x000002, 0xAA);
        bus.write(0x000001, 0xBB);
        let instruction = JMP{addressing_mode: AddressingMode::Absolute};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pc, 0xAABB);
        assert_eq!(registers.cycles, 3);

        // Test a long address
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.pc  = 0x0000;
        registers.pbr  = 0x00;
        bus.write(0x000003, 0xAA);
        bus.write(0x000002, 0xBB);
        bus.write(0x000001, 0xCC);
        let instruction = JMP{addressing_mode: AddressingMode::AbsoluteLong};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pbr, 0xAA);
        assert_eq!(registers.pc, 0xBBCC);
        assert_eq!(registers.cycles, 4);
    }
}
