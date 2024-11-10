use crate::cpu::{bus::Bus, registers::Registers};

use crate::cpu::cycles;
use super::CPUInstruction;
use super::decoder_common;

static INSTR_NAME: &str = "XCE";

pub struct XCE {}

impl CPUInstruction for XCE {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus) {
        let did_mode_change = registers.emulation_mode != registers.get_carry_flag();
        registers.exchange_carry_and_emulation();
        if did_mode_change {
            registers.set_memory_select_flag(true);
            registers.set_index_register_select_flag(true);
        }
        let (bytes, cycles) = cycles::increment_cycles_exchange();
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
        registers.pc = 0x0000;
        let instruction = XCE{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.cycles, 2);
        assert!(registers.get_memory_select_flag());
        assert!(registers.get_index_register_select_flag());

        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.pc = 0x0000;
        registers.emulation_mode = true;
        registers.set_carry_flag(false);
        let instruction = XCE{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.emulation_mode);
        assert!(registers.get_memory_select_flag());
        assert!(registers.get_index_register_select_flag());

        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.pc = 0x0000;
        registers.emulation_mode = false;
        registers.set_carry_flag(true);
        let instruction = XCE{};
        instruction.execute(&mut registers, &mut bus);
        assert_eq!(registers.pc, 0x0001);
        assert_eq!(registers.cycles, 2);
        assert!(registers.emulation_mode);
        assert!(registers.get_memory_select_flag());
        assert!(registers.get_index_register_select_flag());

        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.pc = 0x0000;
        registers.emulation_mode = true;
        registers.set_carry_flag(false);
        let instruction = XCE{};
        instruction.execute(&mut registers, &mut bus);
        assert!(registers.get_carry_flag());
        assert!(registers.get_memory_select_flag());
        assert!(registers.get_index_register_select_flag());
    }
}
