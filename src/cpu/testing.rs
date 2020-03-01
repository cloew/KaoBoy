use super::instruction_context::InstructionContext;
use super::program_counter::ProgramCounter;
use super::registers::registers::Registers;
use super::stack::Stack;
use crate::rc_refcell;
use crate::emulator::Memory;

use std::rc::Rc;
use std::cell::RefCell;


pub fn build_test_instruction_context() -> InstructionContext {
    let memory = rc_refcell!(Memory::new());
    return build_test_instruction_context_with_memory(memory);
}

pub fn build_test_instruction_context_with_memory(memory: Rc<RefCell<Memory>>) -> InstructionContext {
    let program = rc_refcell!(ProgramCounter::new(memory.clone()));
    let registers = rc_refcell!(Registers::new());
    let stack = rc_refcell!(Stack::new(memory.clone()));
    return InstructionContext::new(memory.clone(), program, registers, stack);
}
