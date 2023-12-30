use crate::cpu::{registers::Registers, bus::Bus};

pub fn do_push(registers: &mut Registers, bus: &mut Bus, bytes: &[u8]) {
    for byte in bytes {
        let address = registers.sp as u32;
        bus.write(address, *byte);
        registers.decrement_sp(1);
    }
}
