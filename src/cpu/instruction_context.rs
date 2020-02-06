use super::registers::registers::Registers;

use std::rc::Rc;
use std::cell::RefCell;

pub struct InstructionContext {
    _registers: Rc<RefCell<Registers>>,
}

impl InstructionContext {
	pub fn new(registers: Rc<RefCell<Registers>>) -> InstructionContext {
		return InstructionContext {_registers: registers};
	}
    
    pub fn registers(&self) -> std::cell::Ref<Registers> {
        return self._registers.borrow();
    }
    
    pub fn registers_mut(&self) -> std::cell::RefMut<Registers> {
        return self._registers.borrow_mut();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    pub fn build_test_instruction_context() -> InstructionContext {
        let registers = Rc::new(RefCell::new(Registers::new()));
        return InstructionContext::new(registers);
    }
}
