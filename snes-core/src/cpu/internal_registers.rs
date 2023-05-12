pub const INTERNAL_REGISTERS_ADDRESS: u16 = 0x4200;

pub struct InternalRegisters {
    _registers: [u8; 32],
}

impl InternalRegisters {
    pub fn new() -> Self {
        Self {
            _registers: [0; 32],
        }
    }

    pub fn read(&self, _address: u16) -> u8 {
        // TODO: Placeholder
        // self.registers[(address - INTERNAL_REGISTERS_ADDRESS) as usize]
        0x00
    }

    pub fn write(&mut self, _address: u16, _value: u8) {
        // TODO: Placeholder
        // self.registers[(address - INTERNAL_REGISTERS_ADDRESS) as usize] = value;
    }
}