pub trait CPUInstruction {
    fn execute();
}

pub struct ADC {}
impl CPUInstruction for ADC {
    fn execute () {
    }
}
