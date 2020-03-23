use super::instruction_context::InstructionContext;
use super::program_counter::ProgramCounter;
use super::stack::Stack;
use super::registers::Registers;
use crate::emulator::Memory;
use crate::rc_refcell;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Cpu {
    pub _counter: Rc<RefCell<ProgramCounter>>,
    pub _registers: Rc<RefCell<Registers>>,
    pub _stack: Rc<RefCell<Stack>>,
    
    _context: InstructionContext,
}

impl Cpu {
    pub fn new(memory: Rc<RefCell<Memory>>) -> Cpu {
        let program = rc_refcell!(ProgramCounter::new(memory.clone()));
        let registers = rc_refcell!(Registers::new());
        let stack = rc_refcell!(Stack::new(memory.clone()));
        return Cpu {
            _counter: program.clone(),
            _registers: registers.clone(),
            _stack: stack.clone(),
            _context: InstructionContext::new(
                memory.clone(), program.clone(), registers.clone(), stack.clone()),
        };
    }
    
    pub fn run_next_instruction(&mut self) {
        let instruction = self._counter.borrow_mut().read_next_instruction();
        instruction.run(&mut self._context);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex, rc_refcell};
    
    #[test]
    fn test_run_next_instruction_runs_instruction() {
        const ADD_INSTRUCTION: u8 = 0x87;
        const COUNTER: u16 = 0xABCD;
        const INITIAL_A: u8 = 0x12;
        const EXPECTED_A: u8 = INITIAL_A + INITIAL_A;
        
        let mut memory = Memory::new();
        memory.write_byte(COUNTER, ADD_INSTRUCTION);
        
        let mut cpu = Cpu::new(rc_refcell!(memory));
        cpu._counter.borrow_mut().set_counter(COUNTER);
		cpu._registers.borrow_mut().a.set(INITIAL_A);
        
        cpu.run_next_instruction();
        
        assert_eq!(as_hex!(cpu._registers.borrow().a), as_hex!(EXPECTED_A));
    }
}
