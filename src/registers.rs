macro_rules! get_lower_u8 {
    ($value:expr) => ($value as u8);
}

macro_rules! get_upper_u8 {
    ($value:expr) => ((($value & 0xFF00) >> 8) as u8);
}

macro_rules! build_u16 {
    ($upper:expr, $lower:expr) => (($upper as u16) << 8 | ($lower as u16));
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8, // Special Flags register
    h: u8,
    l: u8,
}

impl Registers {
    fn read_bc_register(&self) -> u16 {
        return build_u16!(self.b, self.c);
    }
    
    fn write_bc_register(&mut self, value: u16) {
        self.b = get_upper_u8!(value); //((value & 0xFF00) >> 8) as u8;
        self.c = get_lower_u8!(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    macro_rules! as_hex {
        ($value:expr) => (format!("0x{:X}", $value));
    }
    
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
}