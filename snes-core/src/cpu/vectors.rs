use super::{interface::CPU, instructions::{phk::PHK, CPUInstruction, php::PHP, push_common}};
use crate::cpu::bus::Bus;


#[allow(clippy::upper_case_acronyms, dead_code)] enum Vector {
    Reset,
    COP,
    Break,
    Abort,
    NMI,
    VBlank,
    IRQ,
    HVTimer,
}

impl Vector {
    pub fn get_base_address(&self) -> u32 {
        match self {
            Self::COP                   => 0x00FFE4,
            Self::Break                 => 0x00FFE6,
            Self::Abort                 => 0x00FFE8,
            Self::NMI | Self::VBlank    => 0x00FFEA,
            Self::Reset                 => 0x00FFFC,
            Self::IRQ | Self::HVTimer   => 0x00FFEE,
        }
    }
}

impl CPU {
    fn get_vector(base_address: u32, bus: &mut Bus) -> u16 {
        (bus.read(base_address) as u16) | ((bus.read(base_address + 1) as u16) << 8)
    }

    pub fn reset_vector(&mut self, bus: &mut Bus) {
        let base_address = Vector::Reset.get_base_address();
        let reset_vector = CPU::get_vector(base_address, bus);
        self.registers.pc = reset_vector;
        self.registers.is_cpu_stopped = false;
    }

    fn get_vector_from_interrupts(&self) -> Option<Vector> {
        if self.registers.get_irq_disable_flag() {
            return None;
        }
        Some(Vector::Reset)
    }

    fn push_emulation_interrupt(&mut self, bus: &mut Bus) {
        if !self.registers.emulation_mode {
            PHK{}.execute(&mut self.registers, bus);
        }
        let values = [
            (self.registers.pc >> 8) as u8,
            self.registers.pc as u8,
        ];
        push_common::do_push(&mut self.registers, bus, &values);
        PHP{}.execute(&mut self.registers, bus);
    }

    pub fn handle_interrupts(&mut self, bus: &mut Bus) {
        self.push_emulation_interrupt(bus);
        if let Some(vector) = self.get_vector_from_interrupts() {
            let effective_vector = vector.get_base_address();
            self.registers.pc = effective_vector as u16;
            self.registers.pbr = (effective_vector >> 16) as u8;
        }
    }
}


#[cfg(test)]
mod cpu_vectors_tests {
    use super::*;

    #[test]
    fn test_reset_vector() {
        let mut cpu = CPU::new();
        let mut bus = Bus::new();
        cpu.registers.is_cpu_stopped = true;
        // TODO: test that the PC register got the right vector
        cpu.reset_vector(&mut bus);
        assert!(!cpu.registers.is_cpu_stopped);
    }
}