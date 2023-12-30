use crate::{cpu::registers::Registers, utils::{num_trait::SnesNum, addressing::AddressingMode, alu}};

pub fn do_bit<T: SnesNum>(registers: &mut Registers, accumulator: T, value: T, addressing_mode: AddressingMode) {
    let (result, _) = alu::and(accumulator, value);
    // Immediate addressing affects only the zero flag
    match addressing_mode {
        AddressingMode::Immediate => registers.set_zero_flag(result.is_zero()),
        _ => {
            registers.set_zero_flag(result.is_zero());
            registers.set_negative_flag(value.is_negative());
            registers.set_overflow_flag(value.next_to_highest_bit());
        }
    };
}
