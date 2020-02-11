use super::instruction_context::InstructionContext;
use super::program_counter::ProgramCounter;
use super::registers::registers::Registers;
use crate::emulator::Memory;

use std::rc::Rc;
use std::cell::RefCell;

pub fn build_test_instruction_context() -> InstructionContext {
    let program = Rc::new(RefCell::new(ProgramCounter::new(Memory::new())));
    let registers = Rc::new(RefCell::new(Registers::new()));
    return InstructionContext::new(program, registers);
}
