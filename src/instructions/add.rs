use crate::registers::Registers;

fn add(registers: &mut Registers, value: u8) {
    let (new_value, overflow) = registers.a.overflowing_add(value);
    
    registers.a = new_value;
    registers.set_zero_flag(new_value == 0);
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
    
    #[test]
    fn test_add_becomes_zero_sets_zero_flag_to_true() {
        const INITIAL_A: u8 = 0x12;
        const VALUE_TO_ADD: u8 = (0x00 as u8).overflowing_sub(INITIAL_A).0;
        
        let mut registers = Registers {a: INITIAL_A, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0};
        
        add(&mut registers, VALUE_TO_ADD);
        
        assert_eq!(registers.get_zero_flag(), true);
    }
    
    #[test]
    fn test_add_becomes_non_zero_sets_zero_flag_to_false() {
        const INITIAL_A: u8 = 0x12;
        const VALUE_TO_ADD: u8 = 0xFF - INITIAL_A;
        
        let mut registers = Registers {a: INITIAL_A, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0};
        
        add(&mut registers, VALUE_TO_ADD);
        
        assert_eq!(registers.get_zero_flag(), false);
    }
}