use crate::{cpu::{registers::Registers, bus::Bus}, utils::addressing::AddressingMode};

pub fn get_effective_address(registers: &Registers, bus: &mut Bus, addressing_mode: AddressingMode) -> u32 {
    addressing_mode.effective_address(
        bus,
        registers.get_pc_address(),
        registers.d,
        registers.sp,
        registers.x, registers.y,
    )
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

pub fn write_8bit_to_address(registers: &mut Registers, bus: &mut Bus, addressing_mode: AddressingMode, value: u8) {
    match addressing_mode {
        AddressingMode::Accumulator => registers.set_low_a(value),
        _ => addressing_mode.store_8bit(
            bus,
            registers.get_pc_address(),
            registers.d,
            registers.sp,
            registers.x, registers.y,
            value,
        ),
    };
}

pub fn write_16bit_to_address(registers: &mut Registers, bus: &mut Bus, addressing_mode: AddressingMode, value: u16) {
    match addressing_mode {
        AddressingMode::Accumulator => registers.a = value,
        _ => addressing_mode.store_16bit(
            bus,
            registers.get_pc_address(),
            registers.d,
            registers.sp,
            registers.x, registers.y,
            value,
        ),
    };
}