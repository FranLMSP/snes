use crate::cpu::bus::Bus;
use crate::cpu::registers::Registers;
use crate::utils::addressing::AddressingMode;

pub mod adc;
pub mod and;
pub mod asl;
pub mod bcc;
pub mod bcs;
pub mod decoder_common;
pub mod branch_common;

pub trait CPUInstruction {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus);
}

pub trait Decode {
    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String;
}

pub fn read_8bit_from_address(registers: &Registers, bus: &mut Bus, addressing_mode: AddressingMode) -> u8 {
    match addressing_mode {
        AddressingMode::Accumulator => registers.a as u8,
        _ => addressing_mode.value_8bit(
            bus,
            registers.get_pc_address(),
            registers.d,
            registers.sp,
            registers.x,
            registers.y,
        )
    }
}

pub fn read_16bit_from_address(registers: &Registers, bus: &mut Bus, addressing_mode: AddressingMode) -> u16 {
    match addressing_mode {
        AddressingMode::Accumulator => registers.a,
        _ => addressing_mode.value_16bit(
            bus,
            registers.get_pc_address(),
            registers.d,
            registers.sp,
            registers.x,
            registers.y,
        )
    }
}
