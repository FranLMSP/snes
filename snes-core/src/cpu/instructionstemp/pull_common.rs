use crate::cpu::{registers::Registers, bus::Bus};


pub fn do_pull(registers: &mut Registers, bus: &mut Bus, count: usize) -> Vec<u8> {
    let mut bytes = vec![];
    let mut is_zero = true;
    for _ in 0..count {
        registers.increment_sp(1);
        let byte = bus.read(registers.sp as u32);
        if byte != 0 {
            is_zero = false;
        }
        bytes.push(byte);
    }
    registers.set_zero_flag(is_zero);
    if bytes.len() > 0 {
        // Low byte is pulled first, so we need to check
        // for the last byte that we pull
        registers.set_negative_flag((bytes[bytes.len() - 1] >> 7) == 1);
    }
    bytes
}