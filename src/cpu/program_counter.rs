use super::instructions::instruction::Instruction;
use super::instructions::instructions::load_instruction;
use crate::emulator::Memory;
use crate::{build_u16};

use std::rc::Rc;
use std::cell::RefCell;

pub struct ProgramCounter {
    _counter: u16,
    pub _memory: Rc<RefCell<Memory>>,
}

impl ProgramCounter {
    pub fn new(memory: Rc<RefCell<Memory>>) -> ProgramCounter {
        return ProgramCounter {_counter: 0, _memory: memory};
    }
    
    pub fn set_counter(&mut self, new_counter: u16) {
        self._counter = new_counter;
    }
    
    pub fn read_next_instruction(&mut self) -> Box<dyn Instruction> {
        let next_byte = self.read_next_byte();
        return load_instruction(next_byte);
    }
    
    pub fn read_next_byte(&mut self) -> u8 {
        let next_byte = self._memory.borrow().read_byte(self._counter);
        self._counter += 1;
        return next_byte;
    }
    
    pub fn read_next_short(&mut self) -> u16 {
        let first_byte = self.read_next_byte();
        let second_byte = self.read_next_byte();
        return build_u16!(first_byte, second_byte);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex, rc_refcell};
    
    fn build_program_counter() -> ProgramCounter {
        return ProgramCounter::new(rc_refcell!(Memory::new()));
    }
    
    #[test]
    fn test_read_next_instruction_returns_instruction() {
        const ADD_INSTRUCTION: u8 = 0x87;
        const COUNTER: u16 = 0xABCD;
        
        let mut program_counter = build_program_counter();
        program_counter.set_counter(COUNTER);
        program_counter._memory.borrow_mut().write_byte(COUNTER, ADD_INSTRUCTION);
        
        program_counter.read_next_instruction();
        
        // Not sure if there'e anything I can assert on
    }
    
    #[test]
    fn test_read_next_instruction_increases_counter() {
        const ADD_INSTRUCTION: u8 = 0x87;
        const COUNTER: u16 = 0xABCD;
        
        let mut program_counter = build_program_counter();
        program_counter.set_counter(COUNTER);
        program_counter._memory.borrow_mut().write_byte(COUNTER, ADD_INSTRUCTION);
        
        program_counter.read_next_instruction();
        
        assert_eq!(program_counter._counter, COUNTER+1);
    }
    
    #[test]
    fn test_read_next_byte_returns_byte() {
        const ADDRESS: u16 = 0xABCD;
        const COUNTER: u16 = ADDRESS;
        const EXPECTED_BYTE: u8 = 0xAB;
        
        let mut program_counter = build_program_counter();
        program_counter.set_counter(COUNTER);
        program_counter._memory.borrow_mut().write_byte(ADDRESS, EXPECTED_BYTE);
        
        let result = program_counter.read_next_byte();
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_BYTE));
    }
    
    #[test]
    fn test_read_next_byte_increases_counter() {
        const ADDRESS: u16 = 0xABCD;
        const COUNTER: u16 = ADDRESS;
        const EXPECTED_BYTE: u8 = 0xAB;
        
        let mut program_counter = build_program_counter();
        program_counter.set_counter(COUNTER);
        program_counter._memory.borrow_mut().write_byte(ADDRESS, EXPECTED_BYTE);
        
        program_counter.read_next_byte();
        
        assert_eq!(program_counter._counter, COUNTER+1);
    }
    
    #[test]
    fn test_read_next_short_returns_short() {
        const ADDRESS: u16 = 0xABCD;
        const COUNTER: u16 = ADDRESS;
        const EXPECTED_SHORT: u16 = 0xFEDC;
        
        let mut program_counter = build_program_counter();
        program_counter.set_counter(COUNTER);
        program_counter._memory.borrow_mut().write_short(ADDRESS, EXPECTED_SHORT);
        
        let result = program_counter.read_next_short();
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_SHORT));
    }
    
    #[test]
    fn test_read_next_short_increases_counter() {
        const ADDRESS: u16 = 0xABCD;
        const COUNTER: u16 = ADDRESS;
        const EXPECTED_SHORT: u16 = 0xFEDC;
        
        let mut program_counter = build_program_counter();
        program_counter.set_counter(COUNTER);
        program_counter._memory.borrow_mut().write_short(ADDRESS, EXPECTED_SHORT);
        
        program_counter.read_next_short();
        
        assert_eq!(program_counter._counter, COUNTER+2);
    }
}