use super::registers::Registers;

pub struct CPU {
    pub registers: Registers,
    pub cycles: usize,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            cycles: 0,
        }
    }
}
