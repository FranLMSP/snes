use crate::{cpu::registers::Registers, utils::{num_trait::SnesNum, alu}, common::flags::Flags};

pub fn do_dec<T: SnesNum>(registers: &mut Registers, target: T) -> T {
    let (result, affected_flags) = alu::sbc_bin(target, T::from_u32(1), false);
    for flag in affected_flags {
        match flag {
            Flags::Negative(_) | Flags::Zero(_) => registers.set_flags(&[flag]),
            _ => {},
        }
    }
    result
}

pub fn do_inc<T: SnesNum>(registers: &mut Registers, target: T) -> T {
    let (result, affected_flags) = alu::adc_bin(target, T::from_u32(1), false);
    for flag in affected_flags {
        match flag {
            Flags::Negative(_) | Flags::Zero(_) => registers.set_flags(&[flag]),
            _ => {},
        }
    }
    result
}
