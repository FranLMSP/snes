/// CPU Status flags for both the 65XX and SPC700 processors

#[derive(Copy, Clone)]
pub enum Flags {
    Negative(bool),
    Overflow(bool),
    MemoryAccumulatorSelect(bool),
    IndexRegisterSelect(bool),
    DecimalMode(bool),
    IRQDisable(bool),
    Zero(bool),
    Carry(bool),
    HalfCarry(bool),
}