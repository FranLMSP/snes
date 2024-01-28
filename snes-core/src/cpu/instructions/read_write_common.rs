use crate::{cpu::{registers::Registers, bus::Bus}, utils::addressing::{AddressingMode, ResolveAddressParams}};

pub fn get_effective_address(registers: &Registers, bus: &mut Bus, addressing_mode: AddressingMode) -> u32 {
    addressing_mode.effective_address(
        bus,
        ResolveAddressParams {
            pc_addr: registers.get_pc_address(),
            direct_page_register: registers.d,
            stack_pointer: registers.sp,
            x: registers.x, y: registers.y,
            dbr: registers.dbr,
        }
    )
}

pub fn read_8bit_from_address(registers: &Registers, bus: &mut Bus, addressing_mode: AddressingMode) -> u8 {
    match addressing_mode {
        AddressingMode::Accumulator => registers.a as u8,
        _ => addressing_mode.read_8bit(
            ResolveAddressParams {
                pc_addr: registers.get_pc_address(),
                direct_page_register: registers.d,
                stack_pointer: registers.sp,
                x: registers.x, y: registers.y,
                dbr: registers.dbr,
            },
            bus,
        )
    }
}

pub fn read_16bit_from_address(registers: &Registers, bus: &mut Bus, addressing_mode: AddressingMode) -> u16 {
    match addressing_mode {
        AddressingMode::Accumulator => registers.a,
        _ => addressing_mode.read_16bit(
            ResolveAddressParams {
                pc_addr: registers.get_pc_address(),
                direct_page_register: registers.d,
                stack_pointer: registers.sp,
                x: registers.x, y: registers.y,
                dbr: registers.dbr,
            },
            bus,
        )
    }
}

pub fn write_8bit_to_address(registers: &mut Registers, bus: &mut Bus, addressing_mode: AddressingMode, value: u8) {
    match addressing_mode {
        AddressingMode::Accumulator => registers.set_low_a(value),
        _ => addressing_mode.write_8bit(
            ResolveAddressParams {
                pc_addr: registers.get_pc_address(),
                direct_page_register: registers.d,
                stack_pointer: registers.sp,
                x: registers.x, y: registers.y,
                dbr: registers.dbr,
            },
            bus,
            value,
        ),
    };
}

pub fn write_16bit_to_address(registers: &mut Registers, bus: &mut Bus, addressing_mode: AddressingMode, value: u16) {
    match addressing_mode {
        AddressingMode::Accumulator => registers.a = value,
        _ => addressing_mode.write_16bit(
            ResolveAddressParams {
                pc_addr: registers.get_pc_address(),
                direct_page_register: registers.d,
                stack_pointer: registers.sp,
                x: registers.x, y: registers.y,
                dbr: registers.dbr,
            },
            bus,
            value,
        ),
    };
}