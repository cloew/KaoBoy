use super::instruction_context::InstructionContext;
use super::program_counter::ProgramCounter;
use super::registers::registers::Registers;
use crate::emulator::Memory;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Cpu {
    _counter: Rc<RefCell<ProgramCounter>>,
    pub _registers: Rc<RefCell<Registers>>,
    
    _context: InstructionContext,
}

impl Cpu {
    pub fn new(memory: Memory) -> Cpu {
        let program = Rc::new(RefCell::new(ProgramCounter::new(memory)));
        let registers = Rc::new(RefCell::new(Registers::new()));
        return Cpu {
            _counter: program.clone(),
            _registers: registers.clone(),
            _context: InstructionContext::new(program.clone(), registers.clone()),
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
    use crate::{as_hex};
    
    #[test]
    fn test_run_next_instruction_runs_instruction() {
        const ADD_INSTRUCTION: u8 = 0x87;
        const COUNTER: u16 = 0xABCD;
        const INITIAL_A: u8 = 0x12;
        const EXPECTED_A: u8 = INITIAL_A + INITIAL_A;
        
        let mut memory = Memory::new();
        memory.set_byte(COUNTER, ADD_INSTRUCTION);
        
        let mut cpu = Cpu::new(memory);
        cpu._counter.borrow_mut().set_counter(COUNTER);
		cpu._registers.borrow_mut().a.set(INITIAL_A);
        
        cpu.run_next_instruction();
        
        assert_eq!(as_hex!(cpu._registers.borrow().a), as_hex!(EXPECTED_A));
    }
}
