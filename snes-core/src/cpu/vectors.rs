use super::{instructions::push_common, interface::CPU, internal_registers::RDNMI};
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

    fn push_interrupt(&mut self, bus: &mut Bus) {
        let pbr = self.registers.pbr;
        if !self.registers.emulation_mode {
            push_common::do_push(&mut self.registers, bus, &[pbr]);
        }
        let values = [
            (self.registers.pc >> 8) as u8,
            self.registers.pc as u8,
        ];
        push_common::do_push(&mut self.registers, bus, &values);
        let p = self.registers.p;
        push_common::do_push(&mut self.registers, bus, &[p]);
    }

    fn handle_interrupt(&mut self, bus: &mut Bus, vector: Vector) {
        self.push_interrupt(bus);
        let base_address = vector.get_base_address();
        let effective_vector = CPU::get_vector(base_address, bus);
        self.registers.pc = effective_vector;
        self.registers.pbr = 0x00;
    }

    pub fn check_interrupts(&mut self, bus: &mut Bus) {
        let rdnmi_byte = bus.read(RDNMI as u32);
        if rdnmi_byte >> 7 != 0 {
            self.registers.is_cpu_waiting_interrupt = false;
            self.handle_interrupt(bus, Vector::NMI);
        }
        if !self.registers.get_irq_disable_flag() && bus.ppu.is_irq_set {
            self.registers.is_cpu_waiting_interrupt = false;
            self.handle_interrupt(bus, Vector::IRQ);
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