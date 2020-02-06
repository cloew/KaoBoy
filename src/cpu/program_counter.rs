use super::instructions::instruction::Instruction;
use super::instructions::instructions::load_instruction;
use super::memory::Memory;

pub struct ProgramCounter {
    _counter: u16,
    pub _memory: Memory,
}

impl ProgramCounter {
    pub fn new(memory: Memory) -> ProgramCounter {
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
        let next_byte = self._memory.read_byte(self._counter);
        self._counter += 1;
        return next_byte;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    
    fn build_program_counter() -> ProgramCounter {
        return ProgramCounter::new(Memory::new());
    }
    
    #[test]
    fn test_read_next_instruction_returns_instruction() {
        const ADD_INSTRUCTION: u8 = 0x87;
        const COUNTER: u16 = 0xABCD;
        
        let mut program_counter = build_program_counter();
        program_counter.set_counter(COUNTER);
        program_counter._memory.set_byte(COUNTER, ADD_INSTRUCTION);
        
        program_counter.read_next_instruction();
        
        // Not sure if there'e anything I can assert on
    }
    
    #[test]
    fn test_read_next_instruction_increases_counter() {
        const ADD_INSTRUCTION: u8 = 0x87;
        const COUNTER: u16 = 0xABCD;
        
        let mut program_counter = build_program_counter();
        program_counter.set_counter(COUNTER);
        program_counter._memory.set_byte(COUNTER, ADD_INSTRUCTION);
        
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
        program_counter._memory.set_byte(ADDRESS, EXPECTED_BYTE);
        
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
        program_counter._memory.set_byte(ADDRESS, EXPECTED_BYTE);
        
        program_counter.read_next_byte();
        
        assert_eq!(program_counter._counter, COUNTER+1);
    }
}