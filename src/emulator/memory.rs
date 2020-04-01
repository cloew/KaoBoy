use crate::{build_u16};

pub struct Memory {
    _memory: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Memory {
        return Memory {_memory: [0; 0xFFFF]};
    }
    
    pub fn bootstrap(&mut self, bootstrap_data: Vec<u8>) {
        for (place, data) in self._memory.iter_mut().zip(bootstrap_data.iter()) {
            *place = *data
        }
    }
    
    pub fn read_byte(&self, address: u16) -> u8 {
        return self._memory[address as usize];
    }
    
    pub fn read_short(&self, address: u16) -> u16 {
        return build_u16!(self.read_byte(address), self.read_byte(address+1));
    }
    
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self._memory[address as usize] = value;
    }
    
    pub fn write_short(&mut self, address: u16, value: u16) {
        let bytes = value.to_be_bytes();
        self.write_byte(address, bytes[0]);
        self.write_byte(address+1, bytes[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    
    #[test]
    fn test_bootstrap_sets_bytes() {
        const BOOTSTRAP_BYTES: [u8; 10] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x16, 0x17, 0x18, 0x1A, 0x1B,
        ];
        let mut memory = Memory::new();
        
        memory.bootstrap(&BOOTSTRAP_BYTES);
        
        for (i, expected) in BOOTSTRAP_BYTES.iter().enumerate() {
            let address: u16 = i as u16;
            assert_eq!(as_hex!(memory.read_byte(address)), as_hex!(expected));
        }
    }
    
    #[test]
    fn test_read_byte_returns_byte() {
        const ADDRESS: u16 = 0xABCD;
        const EXPECTED_BYTE: u8 = 0xAB;
        let mut memory = Memory::new();
        
        memory.write_byte(ADDRESS, EXPECTED_BYTE);
        let result = memory.read_byte(ADDRESS);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_BYTE));
    }
    
    #[test]
    fn test_read_short_returns_short() {
        const ADDRESS: u16 = 0xABCD;
        const EXPECTED_SHORT: u16 = 0xFEDC;
        let mut memory = Memory::new();
        
        memory.write_short(ADDRESS, EXPECTED_SHORT);
        let result = memory.read_short(ADDRESS);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_SHORT));
    }
    
    #[test]
    fn test_write_short_sets_in_proper_order() {
        const ADDRESS: u16 = 0xABCD;
        const EXPECTED_SHORT: u16 = 0xFEDC;
        let bytes = EXPECTED_SHORT.to_be_bytes();
        let EXPECTED_FIRST_BYTE = bytes[0];
        let EXPECTED_SECOND_BYTE = bytes[1];
        let mut memory = Memory::new();
        
        memory.write_short(ADDRESS, EXPECTED_SHORT);
        let firstResult = memory.read_byte(ADDRESS);
        let secondResult = memory.read_byte(ADDRESS+1);
        
        assert_eq!(as_hex!(firstResult), as_hex!(EXPECTED_FIRST_BYTE));
        assert_eq!(as_hex!(secondResult), as_hex!(EXPECTED_SECOND_BYTE));
    }
}