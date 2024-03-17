use crate::{utils::addressing::{AddressingMode, IndexRegister}, cpu::instructions::{adc::ADC, and::AND, asl::ASL, bcc::BCC, bcs::BCS, beq::BEQ, bne::BNE, bmi::BMI, bpl::BPL, bra::BRA, brk::BRK, brl::BRL, bvc::BVC, bvs::BVS, bit::BIT, clc::CLC, cld::CLD, cli::CLI, clv::CLV, cmp::CMP, cop::COP, cpx::CPX, cpy::CPY, dec::DEC, dex::DEX, dey::DEY, eor::EOR, inc::INC, iny::INY, inx::INX, jmp::JMP, jsr::JSR, lda::LDA, ldx::LDX, ldy::LDY, lsr::LSR, mvn::MVN, mvp::MVP, nop::NOP, ora::ORA, pea::PEA, pei::PEI, per::PER, pha::PHA, phb::PHB, phd::PHD, phk::PHK, php::PHP, phx::PHX, phy::PHY, pla::PLA, plb::PLB, pld::PLD, plp::PLP, plx::PLX, ply::PLY, rep::REP, rol::ROL, ror::ROR, rti::RTI, rtl::RTL, rts::RTS, sbc::SBC, sec::SEC, sed::SED, sei::SEI, sep::SEP, sta::STA, stp::STP, stx::STX, sty::STY, stz::STZ, tax::TAX, tay::TAY, tcd::TCD, tcs::TCS, trb::TRB, tsb::TSB, tdc::TDC, tsc::TSC, tsx::TSX, txa::TXA, txs::TXS, txy::TXY, tya::TYA, tyx::TYX, wai::WAI, wdm::WDM, xba::XBA, xce::XCE}};

use super::CPUInstruction;

pub fn map_opcode_to_instruction(opcode: u8) -> Box<dyn CPUInstruction> {
    type A = AddressingMode;
    type I = IndexRegister;
    match opcode {
        // ADC
        0x69 => Box::new(ADC{addressing_mode: A::Immediate}),
        0x6D => Box::new(ADC{addressing_mode: A::Absolute}),
        0x6F => Box::new(ADC{addressing_mode: A::AbsoluteLong}),
        0x65 => Box::new(ADC{addressing_mode: A::DirectPage}),
        0x72 => Box::new(ADC{addressing_mode: A::DirectPageIndirect}),
        0x67 => Box::new(ADC{addressing_mode: A::DirectPageIndirectLong}),
        0x7D => Box::new(ADC{addressing_mode: A::AbsoluteIndexed(I::X)}),
        0x7F => Box::new(ADC{addressing_mode: A::AbsoluteLongIndexed(I::X)}),
        0x79 => Box::new(ADC{addressing_mode: A::AbsoluteIndexed(I::Y)}),
        0x75 => Box::new(ADC{addressing_mode: A::DirectPageIndexed(I::X)}),
        0x61 => Box::new(ADC{addressing_mode: A::DirectPageIndexedIndirect(I::X)}),
        0x71 => Box::new(ADC{addressing_mode: A::DirectPageIndirectIndexed(I::Y)}),
        0x77 => Box::new(ADC{addressing_mode: A::DirectPageIndirectLongIndexed(I::Y)}),
        0x63 => Box::new(ADC{addressing_mode: A::StackRelative}),
        0x73 => Box::new(ADC{addressing_mode: A::StackRelativeIndirectIndexed(I::Y)}),
        // AND
        0x29 => Box::new(AND{addressing_mode: A::Immediate}),
        0x2D => Box::new(AND{addressing_mode: A::Absolute}),
        0x2F => Box::new(AND{addressing_mode: A::AbsoluteLong}),
        0x25 => Box::new(AND{addressing_mode: A::DirectPage}),
        0x32 => Box::new(AND{addressing_mode: A::DirectPageIndirect}),
        0x27 => Box::new(AND{addressing_mode: A::DirectPageIndirectLong}),
        0x3D => Box::new(AND{addressing_mode: A::AbsoluteIndexed(I::X)}),
        0x3F => Box::new(AND{addressing_mode: A::AbsoluteLongIndexed(I::X)}),
        0x39 => Box::new(AND{addressing_mode: A::AbsoluteIndexed(I::Y)}),
        0x35 => Box::new(AND{addressing_mode: A::DirectPageIndexed(I::X)}),
        0x21 => Box::new(AND{addressing_mode: A::DirectPageIndexedIndirect(I::X)}),
        0x31 => Box::new(AND{addressing_mode: A::DirectPageIndirectIndexed(I::Y)}),
        0x37 => Box::new(AND{addressing_mode: A::DirectPageIndirectLongIndexed(I::Y)}),
        0x23 => Box::new(AND{addressing_mode: A::StackRelative}),
        0x33 => Box::new(AND{addressing_mode: A::StackRelativeIndirectIndexed(I::Y)}),
        // ASL
        0x0A => Box::new(ASL{addressing_mode: A::Accumulator}),
        0x0E => Box::new(ASL{addressing_mode: A::Absolute}),
        0x06 => Box::new(ASL{addressing_mode: A::DirectPage}),
        0x1E => Box::new(ASL{addressing_mode: A::AbsoluteIndexed(I::X)}),
        0x16 => Box::new(ASL{addressing_mode: A::DirectPageIndexed(I::X)}),
        // BCC
        0x90 => Box::new(BCC{}),
        // BCS
        0xB0 => Box::new(BCS{}),
        // BEQ
        0xF0 => Box::new(BEQ{}),
        // BNE
        0xD0 => Box::new(BNE{}),
        // BMI
        0x30 => Box::new(BMI{}),
        // BPL
        0x10 => Box::new(BPL{}),
        // BRA
        0x80 => Box::new(BRA{}),
        // BRK
        0x00 => Box::new(BRK{}),
        // BRL
        0x82 => Box::new(BRL{}),
        // BVC
        0x50 => Box::new(BVC{}),
        // BVS
        0x70 => Box::new(BVS{}),
        // BIT
        0x89 => Box::new(BIT{addressing_mode: A::Immediate}),
        0x2C => Box::new(BIT{addressing_mode: A::Absolute}),
        0x24 => Box::new(BIT{addressing_mode: A::DirectPage}),
        0x3C => Box::new(BIT{addressing_mode: A::AbsoluteIndexed(I::X)}),
        0x34 => Box::new(BIT{addressing_mode: A::DirectPageIndexed(I::X)}),
        // CLC
        0x18 => Box::new(CLC{}),
        // CLD
        0xD8 => Box::new(CLD{}),
        // CLI
        0x58 => Box::new(CLI{}),
        // CLV
        0xB8 => Box::new(CLV{}),
        // CMP
        0xC9 => Box::new(CMP{addressing_mode: A::Immediate}),
        0xCD => Box::new(CMP{addressing_mode: A::Absolute}),
        0xCF => Box::new(CMP{addressing_mode: A::AbsoluteLong}),
        0xC5 => Box::new(CMP{addressing_mode: A::DirectPage}),
        0xD2 => Box::new(CMP{addressing_mode: A::DirectPageIndirect}),
        0xC7 => Box::new(CMP{addressing_mode: A::DirectPageIndirectLong}),
        0xDD => Box::new(CMP{addressing_mode: A::AbsoluteIndexed(I::X)}),
        0xDF => Box::new(CMP{addressing_mode: A::AbsoluteLongIndexed(I::X)}),
        0xD9 => Box::new(CMP{addressing_mode: A::AbsoluteIndexed(I::Y)}),
        0xD5 => Box::new(CMP{addressing_mode: A::DirectPageIndexed(I::X)}),
        0xC1 => Box::new(CMP{addressing_mode: A::DirectPageIndexedIndirect(I::X)}),
        0xD1 => Box::new(CMP{addressing_mode: A::DirectPageIndirectIndexed(I::Y)}),
        0xD7 => Box::new(CMP{addressing_mode: A::DirectPageIndirectLongIndexed(I::Y)}),
        0xC3 => Box::new(CMP{addressing_mode: A::StackRelative}),
        0xD3 => Box::new(CMP{addressing_mode: A::StackRelativeIndirectIndexed(I::Y)}),
        // COP
        0x02 => Box::new(COP{}),
        // CPX
        0xE0 => Box::new(CPX{addressing_mode: A::Immediate}),
        0xEC => Box::new(CPX{addressing_mode: A::Absolute}),
        0xE4 => Box::new(CPX{addressing_mode: A::DirectPage}),
        // CPY
        0xC0 => Box::new(CPY{addressing_mode: A::Immediate}),
        0xCC => Box::new(CPY{addressing_mode: A::Absolute}),
        0xC4 => Box::new(CPY{addressing_mode: A::DirectPage}),
        // DEC
        0x3A => Box::new(DEC{addressing_mode: A::Accumulator}),
        0xCE => Box::new(DEC{addressing_mode: A::Absolute}),
        0xC6 => Box::new(DEC{addressing_mode: A::DirectPage}),
        0xDE => Box::new(DEC{addressing_mode: A::AbsoluteIndexed(I::X)}),
        0xD6 => Box::new(DEC{addressing_mode: A::DirectPageIndexed(I::X)}),
        // DEX
        0xCA => Box::new(DEX{}),
        // DEY
        0x88 => Box::new(DEY{}),
        // EOR
        0x49 => Box::new(EOR{addressing_mode: A::Immediate}),
        0x4D => Box::new(EOR{addressing_mode: A::Absolute}),
        0x4F => Box::new(EOR{addressing_mode: A::AbsoluteLong}),
        0x45 => Box::new(EOR{addressing_mode: A::DirectPage}),
        0x52 => Box::new(EOR{addressing_mode: A::DirectPageIndirect}),
        0x47 => Box::new(EOR{addressing_mode: A::DirectPageIndirectLong}),
        0x5D => Box::new(EOR{addressing_mode: A::AbsoluteIndexed(I::X)}),
        0x5F => Box::new(EOR{addressing_mode: A::AbsoluteLongIndexed(I::X)}),
        0x59 => Box::new(EOR{addressing_mode: A::AbsoluteIndexed(I::Y)}),
        0x55 => Box::new(EOR{addressing_mode: A::DirectPageIndexed(I::X)}),
        0x41 => Box::new(EOR{addressing_mode: A::DirectPageIndexedIndirect(I::X)}),
        0x51 => Box::new(EOR{addressing_mode: A::DirectPageIndirectIndexed(I::Y)}),
        0x57 => Box::new(EOR{addressing_mode: A::DirectPageIndirectLongIndexed(I::Y)}),
        0x43 => Box::new(EOR{addressing_mode: A::StackRelative}),
        0x53 => Box::new(EOR{addressing_mode: A::StackRelativeIndirectIndexed(I::Y)}),
        // INC
        0x1A => Box::new(INC{addressing_mode: A::Accumulator}),
        0xEE => Box::new(INC{addressing_mode: A::Absolute}),
        0xE6 => Box::new(INC{addressing_mode: A::DirectPage}),
        0xFE => Box::new(INC{addressing_mode: A::AbsoluteIndexed(I::X)}),
        0xF6 => Box::new(INC{addressing_mode: A::DirectPageIndexed(I::X)}),
        // INX
        0xE8 => Box::new(INX{}),
        // INY
        0xC8 => Box::new(INY{}),
        // JMP
        0x4C => Box::new(JMP{addressing_mode: A::Absolute}),
        0x6C => Box::new(JMP{addressing_mode: A::AbsoluteIndirect}),
        0x7C => Box::new(JMP{addressing_mode: A::AbsoluteIndexedIndirect(I::X)}),
        0x5C => Box::new(JMP{addressing_mode: A::AbsoluteLong}),
        0xDC => Box::new(JMP{addressing_mode: A::AbsoluteIndirectLong}),
        // JSR 
        0x20 => Box::new(JSR{addressing_mode: A::Absolute}),
        0xFC => Box::new(JSR{addressing_mode: A::AbsoluteIndexedIndirect(I::X)}),
        0x22 => Box::new(JSR{addressing_mode: A::AbsoluteLong}), // same as JSL
        // LDA
        0xA9 => Box::new(LDA{addressing_mode: A::Immediate}),
        0xAD => Box::new(LDA{addressing_mode: A::Absolute}),
        0xAF => Box::new(LDA{addressing_mode: A::AbsoluteLong}),
        0xA5 => Box::new(LDA{addressing_mode: A::DirectPage}),
        0xB2 => Box::new(LDA{addressing_mode: A::DirectPageIndirect}),
        0xA7 => Box::new(LDA{addressing_mode: A::DirectPageIndirectLong}),
        0xBD => Box::new(LDA{addressing_mode: A::AbsoluteIndexed(I::X)}),
        0xBF => Box::new(LDA{addressing_mode: A::AbsoluteLongIndexed(I::X)}),
        0xB9 => Box::new(LDA{addressing_mode: A::AbsoluteIndexed(I::Y)}),
        0xB5 => Box::new(LDA{addressing_mode: A::DirectPageIndexed(I::X)}),
        0xA1 => Box::new(LDA{addressing_mode: A::DirectPageIndexedIndirect(I::X)}),
        0xB1 => Box::new(LDA{addressing_mode: A::DirectPageIndirectIndexed(I::Y)}),
        0xB7 => Box::new(LDA{addressing_mode: A::DirectPageIndirectLongIndexed(I::Y)}),
        0xA3 => Box::new(LDA{addressing_mode: A::StackRelative}),
        0xB3 => Box::new(LDA{addressing_mode: A::StackRelativeIndirectIndexed(I::Y)}),
        // LDX
        0xA2 => Box::new(LDX{addressing_mode: A::Immediate}),
        0xAE => Box::new(LDX{addressing_mode: A::Absolute}),
        0xA6 => Box::new(LDX{addressing_mode: A::DirectPage}),
        0xBE => Box::new(LDX{addressing_mode: A::AbsoluteIndexed(I::Y)}),
        0xB6 => Box::new(LDX{addressing_mode: A::DirectPageIndexed(I::Y)}),
        // LDY
        0xA0 => Box::new(LDY{addressing_mode: A::Immediate}),
        0xAC => Box::new(LDY{addressing_mode: A::Absolute}),
        0xA4 => Box::new(LDY{addressing_mode: A::DirectPage}),
        0xBC => Box::new(LDY{addressing_mode: A::AbsoluteIndexed(I::X)}),
        0xB4 => Box::new(LDY{addressing_mode: A::DirectPageIndexed(I::X)}),
        // LSR
        0x4A => Box::new(LSR{addressing_mode: A::Accumulator}),
        0x4E => Box::new(LSR{addressing_mode: A::Absolute}),
        0x46 => Box::new(LSR{addressing_mode: A::DirectPage}),
        0x5E => Box::new(LSR{addressing_mode: A::AbsoluteIndexed(I::X)}),
        0x56 => Box::new(LSR{addressing_mode: A::DirectPageIndexed(I::X)}),
        // MVN
        0x54 => Box::new(MVN{}),
        // MVP
        0x44 => Box::new(MVP{}),
        // NOP
        0xEA => Box::new(NOP{}),
        // ORA
        0x09 => Box::new(ORA{addressing_mode: A::Immediate}),
        0x0D => Box::new(ORA{addressing_mode: A::Absolute}),
        0x0F => Box::new(ORA{addressing_mode: A::AbsoluteLong}),
        0x05 => Box::new(ORA{addressing_mode: A::DirectPage}),
        0x12 => Box::new(ORA{addressing_mode: A::DirectPageIndirect}),
        0x07 => Box::new(ORA{addressing_mode: A::DirectPageIndirectLong}),
        0x1D => Box::new(ORA{addressing_mode: A::AbsoluteIndexed(I::X)}),
        0x1F => Box::new(ORA{addressing_mode: A::AbsoluteLongIndexed(I::X)}),
        0x19 => Box::new(ORA{addressing_mode: A::AbsoluteIndexed(I::Y)}),
        0x15 => Box::new(ORA{addressing_mode: A::DirectPageIndexed(I::X)}),
        0x01 => Box::new(ORA{addressing_mode: A::DirectPageIndexedIndirect(I::X)}),
        0x11 => Box::new(ORA{addressing_mode: A::DirectPageIndirectIndexed(I::Y)}),
        0x17 => Box::new(ORA{addressing_mode: A::DirectPageIndirectLongIndexed(I::Y)}),
        0x03 => Box::new(ORA{addressing_mode: A::StackRelative}),
        0x13 => Box::new(ORA{addressing_mode: A::StackRelativeIndirectIndexed(I::Y)}),
        // PEA
        0xF4 => Box::new(PEA{}),
        // PEI
        0xD4 => Box::new(PEI{}),
        // PER
        0x62 => Box::new(PER{}),
        // PHA
        0x48 => Box::new(PHA{}),
        // PHB
        0x8B => Box::new(PHB{}),
        // PHD
        0x0B => Box::new(PHD{}),
        // PHK
        0x4B => Box::new(PHK{}),
        // PHP
        0x08 => Box::new(PHP{}),
        // PHX
        0xDA => Box::new(PHX{}),
        // PHY
        0x5A => Box::new(PHY{}),
        // PLA
        0x68 => Box::new(PLA{}),
        // PLB
        0xAB => Box::new(PLB{}),
        // PLD
        0x2B => Box::new(PLD{}),
        // PLP
        0x28 => Box::new(PLP{}),
        // PLX
        0xFA => Box::new(PLX{}),
        // PLY
        0x7A => Box::new(PLY{}),
        // REP
        0xC2 => Box::new(REP{}),
        // ROL
        0x2A => Box::new(ROL{addressing_mode: AddressingMode::Accumulator}),
        0x2E => Box::new(ROL{addressing_mode: AddressingMode::Absolute}),
        0x26 => Box::new(ROL{addressing_mode: AddressingMode::DirectPage}),
        0x3E => Box::new(ROL{addressing_mode: AddressingMode::AbsoluteIndexed(I::X)}),
        0x36 => Box::new(ROL{addressing_mode: AddressingMode::DirectPageIndexed(I::X)}),
        // ROR
        0x6A => Box::new(ROR{addressing_mode: AddressingMode::Accumulator}),
        0x6E => Box::new(ROR{addressing_mode: AddressingMode::Absolute}),
        0x66 => Box::new(ROR{addressing_mode: AddressingMode::DirectPage}),
        0x7E => Box::new(ROR{addressing_mode: AddressingMode::AbsoluteIndexed(I::X)}),
        0x76 => Box::new(ROR{addressing_mode: AddressingMode::DirectPageIndexed(I::X)}),
        // RTI
        0x40 => Box::new(RTI{}),
        // RTL
        0x6B => Box::new(RTL{}),
        // RTS
        0x60 => Box::new(RTS{}),
        // SBC
        0xE9 => Box::new(SBC{addressing_mode: A::Immediate}),
        0xED => Box::new(SBC{addressing_mode: A::Absolute}),
        0xEF => Box::new(SBC{addressing_mode: A::AbsoluteLong}),
        0xE5 => Box::new(SBC{addressing_mode: A::DirectPage}),
        0xF2 => Box::new(SBC{addressing_mode: A::DirectPageIndirect}),
        0xE7 => Box::new(SBC{addressing_mode: A::DirectPageIndirectLong}),
        0xFD => Box::new(SBC{addressing_mode: A::AbsoluteIndexed(I::X)}),
        0xFF => Box::new(SBC{addressing_mode: A::AbsoluteLongIndexed(I::X)}),
        0xF9 => Box::new(SBC{addressing_mode: A::AbsoluteIndexed(I::Y)}),
        0xF5 => Box::new(SBC{addressing_mode: A::DirectPageIndexed(I::X)}),
        0xE1 => Box::new(SBC{addressing_mode: A::DirectPageIndexedIndirect(I::X)}),
        0xF1 => Box::new(SBC{addressing_mode: A::DirectPageIndirectIndexed(I::Y)}),
        0xF7 => Box::new(SBC{addressing_mode: A::DirectPageIndirectLongIndexed(I::Y)}),
        0xE3 => Box::new(SBC{addressing_mode: A::StackRelative}),
        0xF3 => Box::new(SBC{addressing_mode: A::StackRelativeIndirectIndexed(I::Y)}),
        // SEC
        0x38 => Box::new(SEC{}),
        // SED
        0xF8 => Box::new(SED{}),
        // SEI
        0x78 => Box::new(SEI{}),
        // SEP
        0xE2 => Box::new(SEP{}),
        // STA
        0x8D => Box::new(STA{addressing_mode: A::Absolute}),
        0x8F => Box::new(STA{addressing_mode: A::AbsoluteLong}),
        0x85 => Box::new(STA{addressing_mode: A::DirectPage}),
        0x92 => Box::new(STA{addressing_mode: A::DirectPageIndirect}),
        0x87 => Box::new(STA{addressing_mode: A::DirectPageIndirectLong}),
        0x9D => Box::new(STA{addressing_mode: A::AbsoluteIndexed(I::X)}),
        0x9F => Box::new(STA{addressing_mode: A::AbsoluteLongIndexed(I::X)}),
        0x99 => Box::new(STA{addressing_mode: A::AbsoluteIndexed(I::Y)}),
        0x95 => Box::new(STA{addressing_mode: A::DirectPageIndexed(I::X)}),
        0x81 => Box::new(STA{addressing_mode: A::DirectPageIndexedIndirect(I::X)}),
        0x91 => Box::new(STA{addressing_mode: A::DirectPageIndirectIndexed(I::Y)}),
        0x97 => Box::new(STA{addressing_mode: A::DirectPageIndirectLongIndexed(I::Y)}),
        0x83 => Box::new(STA{addressing_mode: A::StackRelative}),
        0x93 => Box::new(STA{addressing_mode: A::StackRelativeIndirectIndexed(I::Y)}),
        // STP
        0xDB => Box::new(STP{}),
        // STX
        0x8E => Box::new(STX{addressing_mode: A::Absolute}),
        0x86 => Box::new(STX{addressing_mode: A::DirectPage}),
        0x96 => Box::new(STX{addressing_mode: A::DirectPageIndexed(I::Y)}),
        // STY
        0x8C => Box::new(STY{addressing_mode: A::Absolute}),
        0x84 => Box::new(STY{addressing_mode: A::DirectPage}),
        0x94 => Box::new(STY{addressing_mode: A::DirectPageIndexed(I::X)}),
        // STZ
        0x9C => Box::new(STZ{addressing_mode: A::Absolute}),
        0x64 => Box::new(STZ{addressing_mode: A::DirectPage}),
        0x9E => Box::new(STZ{addressing_mode: A::AbsoluteIndexed(I::X)}),
        0x74 => Box::new(STZ{addressing_mode: A::DirectPageIndexed(I::X)}),
        // TAX
        0xAA => Box::new(TAX{}),
        // TAY
        0xA8 => Box::new(TAY{}),
        // TCD
        0x5B => Box::new(TCD{}),
        // TCS
        0x1B => Box::new(TCS{}),
        // TDC
        0x7B => Box::new(TDC{}),
        // TRB
        0x1C => Box::new(TRB{addressing_mode: A::Absolute}),
        0x14 => Box::new(TRB{addressing_mode: A::DirectPage}),
        // TSB
        0x0C => Box::new(TSB{addressing_mode: A::Absolute}),
        0x04 => Box::new(TSB{addressing_mode: A::DirectPage}),
        // TSC
        0x3B => Box::new(TSC{}),
        // TSX
        0xBA => Box::new(TSX{}),
        // TXA
        0x8A => Box::new(TXA{}),
        // TXS
        0x9A => Box::new(TXS{}),
        // TXY
        0x9B => Box::new(TXY{}),
        // TYA
        0x98 => Box::new(TYA{}),
        // TYX
        0xBB => Box::new(TYX{}),
        // WAI
        0xCB => Box::new(WAI{}),
        // WDM
        0x42 => Box::new(WDM{}),
        // XBA
        0xEB => Box::new(XBA{}),
        // XCE
        0xFB => Box::new(XCE{}),
    }
}