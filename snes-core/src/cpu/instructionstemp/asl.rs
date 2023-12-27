use crate::{cpu::{bus::Bus, registers::Registers}, utils::{alu, addressing::AddressingMode}};

use crate::cpu::cycles;
use super::{CPUInstruction, Decode};

pub const OPCODE: u8 = 0x0A;

fn mnemonic_8bit(registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    format!("{:02X} {:02X} __ __ | ASL #${:02X}", OPCODE, next_byte, next_byte)
}

fn mnemonic_16bit(registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    let next_second_byte = bus.read_external(registers.get_pc_address() + 2);
    let word = (next_byte as u16) | ((next_byte as u16) << 8);
    format!("{:02X} {:02X} {:02X} __ | AND #${:04X}", OPCODE, next_byte, next_second_byte, word)
}

pub struct ASL8 {}

impl CPUInstruction for ASL8 {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus, addressing_mode: AddressingMode) {
        let (result, affected_flags) = alu::asl(
            registers.a as u8,
        );
        registers.set_low_a(result);
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_bitwise(&registers, addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for ASL8 {
    fn mnemonic(&self, registers: &Registers, bus: &Bus) -> String {
        mnemonic_8bit(registers, bus)
    }
}

pub struct ASL16 {}

impl CPUInstruction for ASL16 {
    fn execute(&self, registers: &mut Registers, _bus: &mut Bus, addressing_mode: AddressingMode) {
        let (result, affected_flags) = alu::asl(
            registers.a,
        );
        registers.a = result;
        registers.set_flags(&affected_flags);
        let (bytes, cycles) = cycles::increment_cycles_bitwise(&registers, addressing_mode);
        registers.increment_pc(bytes); registers.cycles += cycles;
    }
}

impl Decode for ASL16 {
    fn mnemonic(&self, registers: &Registers, bus: &Bus) -> String {
        mnemonic_16bit(registers, bus)
    }
}


#[cfg(test)]
mod cpu_instructions_tests {
    use super::*;

    #[test]
    fn test_asl_8bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0b01010000;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(true);
        let instruction = ASL8{};
        instruction.execute(&mut registers, &mut bus, AddressingMode::Immediate);
        assert_eq!(registers.a, 0b10100000);
        assert_eq!(registers.pc, 0x02);
        assert_eq!(registers.cycles, 2);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_zero_flag());
        assert!(registers.get_negative_flag());
    }

    #[test]
    fn test_asl_16bit() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        registers.emulation_mode = false;
        registers.a   = 0b01010000_00000000;
        registers.pbr = 0x00;
        registers.pc  = 0x0000;
        registers.set_memory_select_flag(false);
        let instruction = ASL16{};
        instruction.execute(&mut registers, &mut bus, AddressingMode::Immediate);
        assert_eq!(registers.a, 0b10100000_00000000);
        assert_eq!(registers.pc, 0x03);
        assert_eq!(registers.cycles, 3);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_zero_flag());
        assert!(registers.get_negative_flag());
    }
}
