use crate::registers::Registers;

fn add(registers: &mut Registers, value: u8) {
    registers.a += value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    
    #[test]
    fn test_add_adds_to_a_register() {
        const INITIAL_A: u8 = 0x12;
        const VALUE_TO_ADD: u8 = 0x34;
        const EXPECTED_VALUE: u16 = 0x46;
        
        let mut registers = Registers {a: INITIAL_A, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0};
        
        add(&mut registers, VALUE_TO_ADD);
        
        assert_eq!(as_hex!(registers.a), as_hex!(EXPECTED_VALUE));
    }
}