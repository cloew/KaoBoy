use super::instruction_context::InstructionContext;
use super::program_counter::ProgramCounter;
use super::registers::registers::Registers;
use crate::rc_refcell;
use crate::emulator::Memory;


pub fn build_test_instruction_context() -> InstructionContext {
    let memory = rc_refcell!(Memory::new());
    let program = rc_refcell!(ProgramCounter::new(memory));
    let registers = rc_refcell!(Registers::new());
    return InstructionContext::new(program, registers);
}
