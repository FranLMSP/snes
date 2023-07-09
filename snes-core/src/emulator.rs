use crate::cpu::CPU;
use crate::cpu::bus::Bus;
use crate::rom::ROM;
use crate::rom::lo_rom::LoROM;

pub struct Emulator {
    pub cpu: CPU,
    pub bus: Bus,
    pub rom: Box<dyn ROM>,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            bus: Bus::new(),
            rom: Box::new(LoROM::new()),
        }
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.bus);
        self.bus.ppu.tick(self.cpu.cycles);

        self.cpu.cycles = 0;
    }

    pub fn is_frame_ending(&self) -> bool {
        self.bus.ppu.registers.v_count == 0
    }

    pub fn is_frame_starting(&self) -> bool {
        self.bus.ppu.registers.v_count == 1
    }

    pub fn reset(&mut self) {
        self.cpu.reset_vector(&mut self.bus);
    }
}