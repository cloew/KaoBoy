use super::program_counter::ProgramCounter;
use super::registers::registers::Registers;

use std::rc::Rc;
use std::cell::RefCell;

pub struct InstructionContext {
    _program: Rc<RefCell<ProgramCounter>>,
    _registers: Rc<RefCell<Registers>>,
}

impl InstructionContext {
	pub fn new(program: Rc<RefCell<ProgramCounter>>, registers: Rc<RefCell<Registers>>) -> InstructionContext {
		return InstructionContext {_program: program, _registers: registers};
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
}
