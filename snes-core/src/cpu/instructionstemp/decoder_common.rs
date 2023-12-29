use crate::cpu::registers::Registers;
use crate::cpu::bus::Bus;
use crate::utils::addressing::{IndexRegister, AddressingMode};

pub fn mnemonic_arithmetic(is_16bit: bool, opcode: u8, instr_name: &str, addressing_mode: AddressingMode, registers: &Registers, bus: &Bus) -> String {
    type A = AddressingMode;
    match addressing_mode {
        A::Accumulator => mnemonic_accumulator(opcode, instr_name),
        A::Immediate => match is_16bit {
            true =>  mnemonic_16bit_immediate(opcode, instr_name, registers, bus),
            false => mnemonic_8bit_immediate(opcode, instr_name, registers, bus),
        },
        A::Absolute => mnemonic_absolute(opcode, instr_name, registers, bus),
        A::AbsoluteLong => mnemonic_absolute_long(opcode, instr_name, registers, bus),
        A::DirectPage => mnemonic_direct_page(opcode, instr_name, registers, bus),
        A::DirectPageIndirect => mnemonic_direct_page_indirect(opcode, instr_name, registers, bus),
        A::DirectPageIndirectLong => mnemonic_direct_page_indirect_long(opcode, instr_name, registers, bus),
        A::AbsoluteIndexed(idx) => mnemonic_absolute_indexed(opcode, instr_name, idx, registers, bus),
        A::AbsoluteLongIndexed(idx) => mnemonic_absolute_long_indexed(opcode, instr_name, idx, registers, bus),
        A::DirectPageIndexed(idx) => mnemonic_direct_page_indexed(opcode, instr_name, idx, registers, bus),
        A::DirectPageIndexedIndirect(idx) => mnemonic_direct_page_indexed_indirect(opcode, instr_name, idx, registers, bus),
        A::DirectPageIndirectLongIndexed(idx) => mnemonic_direct_page_indirect_long_indexed(opcode, instr_name, idx, registers, bus),
        A::StackRelative => mnemonic_stack_relative(opcode, instr_name, registers, bus),
        A::StackRelativeIndirectIndexed(idx)=> mnemonic_stack_relative_indirect_indexed(opcode, instr_name, idx, registers, bus),
        _ => unreachable!(),
    }
}

pub fn mnemonic_accumulator(opcode: u8, instr_name: &str) -> String {
    format!("{:02X} __ __ __ | {} A", opcode, instr_name)
}

pub fn mnemonic_8bit_immediate(opcode: u8, instr_name: &str, registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    format!("{:02X} {:02X} __ __ | {} #${:04X}", opcode, next_byte, instr_name, next_byte)
}

pub fn mnemonic_16bit_immediate(opcode: u8, instr_name: &str, registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    let next_second_byte = bus.read_external(registers.get_pc_address() + 2);
    let word = (next_byte as u16) | ((next_byte as u16) << 8);
    format!("{:02X} {:02X} {:02X} __ | {} #${:04X}", opcode, next_byte, next_second_byte, instr_name, word)
}

pub fn mnemonic_absolute(opcode: u8, instr_name: &str, registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    let next_second_byte = bus.read_external(registers.get_pc_address() + 2);
    let word = (next_byte as u16) | ((next_byte as u16) << 8);
    format!("{:02X} {:02X} {:02X} __ | {} ${:04X}", opcode, next_byte, next_second_byte, instr_name, word)
}

pub fn mnemonic_absolute_long(opcode: u8, instr_name: &str, registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    let next_second_byte = bus.read_external(registers.get_pc_address() + 2);
    let next_third_byte = bus.read_external(registers.get_pc_address() + 3);
    let word_long = (next_byte as u32) | ((next_second_byte as u32) << 8) | ((next_third_byte as u32) << 16);
    format!("{:02X} {:02X} {:02X} {:02X} | {} ${:06X}", opcode, next_byte, next_second_byte, next_third_byte, instr_name, word_long)
}

pub fn mnemonic_direct_page(opcode: u8, instr_name: &str, registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    format!("{:02X} {:02X} __ __ | {} ${:02X} | dp", opcode, next_byte, instr_name, next_byte)
}

pub fn mnemonic_direct_page_indirect(opcode: u8, instr_name: &str, registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    format!("{:02X} {:02X} __ __ | {} (${:02X})", opcode, next_byte, instr_name, next_byte)
}

pub fn mnemonic_direct_page_indirect_long(opcode: u8, instr_name: &str, registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    format!("{:02X} {:02X} __ __ | {} [${:02X}]", opcode, next_byte, instr_name, next_byte)
}

pub fn mnemonic_absolute_indexed(opcode: u8, instr_name: &str, index: IndexRegister, registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    let next_second_byte = bus.read_external(registers.get_pc_address() + 2);
    let word = (next_byte as u16) | ((next_byte as u16) << 8);
    format!("{:02X} {:02X} {:02X} __ | {} ${:04X}, {}", opcode, next_byte, next_second_byte, instr_name, word, index)
}

pub fn mnemonic_absolute_long_indexed(opcode: u8, instr_name: &str, index: IndexRegister, registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    let next_second_byte = bus.read_external(registers.get_pc_address() + 2);
    let next_third_byte = bus.read_external(registers.get_pc_address() + 3);
    let word_long = (next_byte as u32) | ((next_second_byte as u32) << 8) | ((next_third_byte as u32) << 16);
    format!("{:02X} {:02X} {:02X} {:02X} | {} ${:06X}, {}", opcode, next_byte, next_second_byte, next_third_byte, instr_name, word_long, index)
}

pub fn mnemonic_direct_page_indexed(opcode: u8, instr_name: &str, index: IndexRegister, registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    format!("{:02X} {:02X} __ __ | {} ${:02X}, {} | dp", opcode, next_byte, instr_name, next_byte, index)
}

pub fn mnemonic_direct_page_indexed_indirect(opcode: u8, instr_name: &str, index: IndexRegister, registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    format!("{:02X} {:02X} __ __ | {} (${:02X}, {}) | dp", opcode, next_byte, instr_name, next_byte, index)
}

pub fn mnemonic_direct_page_indirect_long_indexed(opcode: u8, instr_name: &str, index: IndexRegister, registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    format!("{:02X} {:02X} __ __ | {} [${:02X}], {} | dp", opcode, next_byte, instr_name, next_byte, index)
}

pub fn mnemonic_stack_relative(opcode: u8, instr_name: &str, registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    format!("{:02X} {:02X} __ __ | {} ${:02X}, S", opcode, next_byte, instr_name, next_byte)
}

pub fn mnemonic_stack_relative_indirect_indexed(opcode: u8, instr_name: &str, index: IndexRegister, registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    format!("{:02X} {:02X} __ __ | {} (${:02X}, S), {}", opcode, next_byte, instr_name, next_byte, index)
}

pub fn mnemonic_branch_nearlabel(opcode: u8, instr_name: &str, registers: &Registers, bus: &Bus) -> String {
    let nearlabel = bus.read_external(registers.get_pc_address() + 1);
    format!("{:02X} {:02X} __ __ | {} ${:02X}", opcode, nearlabel, instr_name, nearlabel)
}

pub fn mnemonic_single_byte_instr(opcode: u8, instr_name: &str) -> String {
    format!("{:02X} __ __ __ | {}", opcode, instr_name)
}

pub fn mnemonic_move(opcode: u8, instr_name: &str, registers: &Registers, bus: &Bus) -> String {
    let next_byte = bus.read_external(registers.get_pc_address() + 1);
    let next_second_byte = bus.read_external(registers.get_pc_address() + 2);
    format!("{:02X} {:02X} {:02X} __ | {} {:02X},{:02X}", opcode, next_byte, next_second_byte, instr_name, next_second_byte, next_byte)
}
