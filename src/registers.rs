use crate::{build_u16, get_lower_u8, get_upper_u8};

const ZERO_FLAG_MASK: u8 = 0x80;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8, // Special Flags register
    pub h: u8,
    pub l: u8,
}

impl Registers {
    fn read_bc_register(&self) -> u16 {
        return build_u16!(self.b, self.c);
    }
    
    fn write_bc_register(&mut self, value: u16) {
        self.b = get_upper_u8!(value); //((value & 0xFF00) >> 8) as u8;
        self.c = get_lower_u8!(value);
    }
    
    pub fn get_zero_flag(&self) -> bool {
        return self.f & ZERO_FLAG_MASK > 0;
    }
    
    pub fn set_zero_flag(&mut self, is_zero: bool) {
        if is_zero {
            self.f |= ZERO_FLAG_MASK;
        } else {
            self.f &= !ZERO_FLAG_MASK;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    
    #[test]
    fn test_read_bc_register_combines_underlying_registers() {
        let mut registers = Registers {a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0};
        const EXPECTED_VALUE: u16 = 0x1234;
        
        registers.b = 0x12;
        registers.c = 0x34;
        
        assert_eq!(as_hex!(registers.read_bc_register()), as_hex!(EXPECTED_VALUE));
    }
    
    #[test]
    fn test_write_bc_register_separates_to_underlying_registers() {
        let mut registers = Registers {a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0};
        const EXPECTED_B_VALUE: u8 = 0x12;
        const EXPECTED_C_VALUE: u8 = 0x34;
        
        registers.write_bc_register(0x1234);
        
        assert_eq!(as_hex!(registers.b), as_hex!(EXPECTED_B_VALUE));
        assert_eq!(as_hex!(registers.c), as_hex!(EXPECTED_C_VALUE));
    }
    
    #[test]
    fn test_get_zero_flag_true_reads_flag_properly() {
        const INITIAL_FLAGS: u8 = 0x80;
        
        let mut registers = Registers {a: 0, b: 0, c: 0, d: 0, e: 0, f: INITIAL_FLAGS, h: 0, l: 0};
        
        let zero_flag = registers.get_zero_flag();
        
        assert_eq!(zero_flag, true);
    }
    
    #[test]
    fn test_get_zero_flag_false_reads_flag_properly() {
        const INITIAL_FLAGS: u8 = 0x70;
        
        let mut registers = Registers {a: 0, b: 0, c: 0, d: 0, e: 0, f: INITIAL_FLAGS, h: 0, l: 0};
        
        let zero_flag = registers.get_zero_flag();
        
        assert_eq!(zero_flag, false);
    }
    
    #[test]
    fn test_set_zero_flag_to_true_flags_only_the_zero_bit() {
        const INITIAL_FLAGS: u8 = 0x70;
        const EXPECTED_FLAGS: u8 = 0xF0;
        
        let mut registers = Registers {a: 0, b: 0, c: 0, d: 0, e: 0, f: INITIAL_FLAGS, h: 0, l: 0};
        
        registers.set_zero_flag(true);
        
        assert_eq!(as_hex!(registers.f), as_hex!(EXPECTED_FLAGS));
    }
    
    #[test]
    fn test_set_zero_flag_to_false_flags_only_the_zero_bit() {
        const INITIAL_FLAGS: u8 = 0xF0;
        const EXPECTED_FLAGS: u8 = 0x70;
        
        let mut registers = Registers {a: 0, b: 0, c: 0, d: 0, e: 0, f: INITIAL_FLAGS, h: 0, l: 0};
        
        registers.set_zero_flag(false);
        
        assert_eq!(as_hex!(registers.f), as_hex!(EXPECTED_FLAGS));
    }
}