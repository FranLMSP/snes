use super::cpu::CPU;
use crate::cpu::bus::Bus;

impl CPU {
    pub fn reset_vector(&mut self, bus: &Bus) {
        let reset_vector = (bus.read(0x00FFFC) as u16) | ((bus.read(0x00FFFD) as u16) << 8);
        self.registers.pc = reset_vector;
        self.is_stopped = false;
    }
}


#[cfg(test)]
mod cpu_vectors_tests {
    use super::*;

    #[test]
    fn test_reset_vector() {
        let mut cpu = CPU::new();
        let bus = Bus::new();
        cpu.is_stopped = true;
        // TODO: test that the PC register got the right vector
        cpu.reset_vector(&bus);
        assert_eq!(cpu.is_stopped, false);
    }
}