
pub struct Memory {
    _memory: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Memory {
        return Memory {_memory: [0; 0xFFFF]};
    }
    
    pub fn read_byte(&self, address: u16) -> u8 {
        return self._memory[address as usize];
    }
    
    pub fn set_byte(&mut self, address: u16, value: u8) {
        self._memory[address as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    
    #[test]
    fn test_read_byte_returns_byte() {
        const ADDRESS: u16 = 0xABCD;
        const EXPECTED_BYTE: u8 = 0xAB;
        let mut memory = Memory::new();
        
        memory.set_byte(ADDRESS, EXPECTED_BYTE);
        let result = memory.read_byte(ADDRESS);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_BYTE));
    }
}