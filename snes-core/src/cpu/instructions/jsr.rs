use crate::cpu::{bus::Bus, registers::Registers};
use crate::utils::addressing::{AddressingMode, IndexRegister};

use super::read_write_common::get_effective_address;
use super::{CPUInstruction, push_common};
use super::decoder_common;
use crate::cpu::cycles;

static INSTR_NAME: &str = "JSR";

pub struct JSR {
    pub addressing_mode: AddressingMode,
}

impl CPUInstruction for JSR {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus) {
        let effective_address = get_effective_address(registers, bus, self.addressing_mode);
        let is_long = matches!(
            self.addressing_mode, AddressingMode::AbsoluteLong |
            AddressingMode::AbsoluteIndirectLong |
            AddressingMode::AbsoluteIndexedIndirect(IndexRegister::X) |
            AddressingMode::AbsoluteIndexedIndirect(IndexRegister::Y)
        );
        // We need to push the *next* instruction onto the stack
        let (bytes, cycles) = cycles::increment_cycles_jsr(self.addressing_mode);
        registers.increment_pc(bytes - 1); registers.cycles += cycles;
        let value = registers.get_pc_address();
        if is_long {
            push_common::do_push(registers, bus, &[
                (value >> 16) as u8,
                (value >> 8) as u8,
                value as u8,
            ]);
        } else {
            push_common::do_push(registers, bus, &[
                (value >> 8) as u8,
                value as u8,
            ]);
        }
        registers.pc = effective_address as u16;
        if is_long {
            registers.pbr = (effective_address >> 16) as u8;
        }
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
        // Test a long address
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.pc  = 0x1234;
        registers.pbr  = 0x00;
        registers.sp  = 0x1FC;
        bus.write(registers.get_pc_address() + 3, 0xAA);
        bus.write(registers.get_pc_address() + 2, 0xBB);
        bus.write(registers.get_pc_address() + 1, 0xCC);
        // write next instruction
        let instruction = JSR{addressing_mode: AddressingMode::AbsoluteLong};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(bus.read(0x1FC), 0x00);
        assert_eq!(bus.read(0x1FB), 0x12);
        assert_eq!(bus.read(0x1FA), 0x37); // we should store the NEXT instruction
        assert_eq!(registers.pbr, 0xAA);
        assert_eq!(registers.pc, 0xBBCC);
        assert_eq!(registers.sp, 0x1F9);
        assert_eq!(registers.cycles, 8);
    }
}
