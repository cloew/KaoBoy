use super::program_counter::ProgramCounter;
use super::registers::registers::Registers;
use super::stack::Stack;
use crate::emulator::Memory;

use std::rc::Rc;
use std::cell::RefCell;

pub struct InstructionContext {
    _memory: Rc<RefCell<Memory>>,
    _program: Rc<RefCell<ProgramCounter>>,
    _registers: Rc<RefCell<Registers>>,
    _stack: Rc<RefCell<Stack>>,
}

impl InstructionContext {
	pub fn new(
            memory: Rc<RefCell<Memory>>,
            program: Rc<RefCell<ProgramCounter>>,
            registers: Rc<RefCell<Registers>>,
            stack: Rc<RefCell<Stack>>) -> InstructionContext {
		return InstructionContext {
            _memory: memory,
            _program: program,
            _registers: registers,
            _stack: stack};
	}
    
    pub fn memory(&self) -> std::cell::Ref<Memory> {
        return self._memory.borrow();
    }
    
    pub fn memory_mut(&self) -> std::cell::RefMut<Memory> {
        return self._memory.borrow_mut();
    }
    
    pub fn program_mut(&self) -> std::cell::RefMut<ProgramCounter> {
        return self._program.borrow_mut();
    }
    
    pub fn registers(&self) -> std::cell::Ref<Registers> {
        return self._registers.borrow();
    }
    
    pub fn registers_mut(&self) -> std::cell::RefMut<Registers> {
        return self._registers.borrow_mut();
    }
    
    pub fn stack_mut(&self) -> std::cell::RefMut<Stack> {
        return self._stack.borrow_mut();
    }
}
