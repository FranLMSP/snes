use crate::cpu::bus::Bus;
use crate::cpu::registers::Registers;
use crate::utils::addressing::AddressingMode;

pub mod adc;
pub mod and;
pub mod asl;
pub mod bcc;
pub mod bcs;
pub mod beq;
pub mod bne;
pub mod bmi;
pub mod bpl;
pub mod bra;
pub mod brk;
pub mod brl;
pub mod bvc;
pub mod bvs;
pub mod bit;
pub mod clc;
pub mod cld;
pub mod cli;
pub mod clv;
pub mod cmp;
pub mod cop;
pub mod cpx;
pub mod cpy;
pub mod dec;
pub mod dex;
pub mod dey;
pub mod eor;
pub mod inc;
pub mod inx;
pub mod iny;
pub mod jmp;
pub mod jsr;
pub mod lda;
pub mod ldx;
pub mod ldy;
pub mod lsr;
pub mod mvn;
pub mod mvp;
pub mod nop;
pub mod ora;
pub mod pea;
pub mod pei;
pub mod per;
pub mod pha;
pub mod phb;
pub mod phd;
pub mod phk;
pub mod php;
pub mod phx;
pub mod phy;
pub mod pla;
pub mod plb;
pub mod pld;
pub mod plp;
pub mod plx;
pub mod ply;
pub mod rep;
pub mod bit_common;
pub mod dec_common;
pub mod decoder_common;
pub mod branch_common;
pub mod push_common;
pub mod pull_common;
pub mod comp_common;
pub mod move_common;

pub trait CPUInstruction {
    fn execute(&self, registers: &mut Registers, bus: &mut Bus);
}

pub trait Decode {
    fn mnemonic(&self, registers: &Registers, bus: &Bus, opcode: u8) -> String;
}

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
