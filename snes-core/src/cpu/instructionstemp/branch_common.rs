use crate::cpu::registers::Registers;

pub fn do_branch(nearlabel: u8, registers: &mut Registers) -> bool {
    let is_negative = (nearlabel >> 7) != 0;
    let old_pc = registers.get_pc_address();
    if is_negative {
        let nearlabel = !nearlabel + 1;
        registers.decrement_pc(nearlabel as u16);
    } else {
        registers.increment_pc(nearlabel as u16);
    }
    let new_pc = registers.get_pc_address();
    let page_boundary_crossed = (old_pc & 0xFF00) != (new_pc & 0xFF00);
    return page_boundary_crossed
}
