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

    pub fn run(&mut self) {
        self.cpu.run(&mut self.bus);
        self.bus.ppu.tick(self.cpu.cycles);

        self.cpu.cycles = 0;
    }
}