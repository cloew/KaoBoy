use crate::cpu::memory::{Memory};

pub struct ProgramCounter {
    _counter: u16,
    _memory: Memory,
}

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        return ProgramCounter {_counter: 0, _memory: Memory::new()};
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
    use crate::{as_hex};
    
    #[test]
    fn test_read_next_byte_returns_byte() {
        const ADDRESS: u16 = 0xABCD;
        const COUNTER: u16 = ADDRESS;
        const EXPECTED_BYTE: u8 = 0xAB;
        let mut program_counter = ProgramCounter::new();
        
        program_counter._counter = COUNTER;
        program_counter._memory.set_byte(ADDRESS, EXPECTED_BYTE);
        let result = program_counter.read_next_byte();
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_BYTE));
    }
}