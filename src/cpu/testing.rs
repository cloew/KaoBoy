use super::instruction_context::InstructionContext;
use super::registers::registers::Registers;

use std::rc::Rc;
use std::cell::RefCell;

pub fn build_test_instruction_context() -> InstructionContext {
    let registers = Rc::new(RefCell::new(Registers::new()));
    return InstructionContext::new(registers);
}
