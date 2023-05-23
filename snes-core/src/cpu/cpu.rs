use super::registers::Registers;

pub struct CPU {
    pub registers: Registers,
    pub cycles: usize,
    pub is_stopped: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            cycles: 0,
            is_stopped: false,
        }
    }
}
