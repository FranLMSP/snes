use super::registers::Registers;

pub struct CPU {
    pub registers: Registers,
    pub cycles: usize, // TODO: remove cycles from here
    pub is_stopped: bool, // TODO: remove from here
    pub is_waiting_interrupt: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            cycles: 0,
            is_stopped: false,
            is_waiting_interrupt: false,
        }
    }
}
