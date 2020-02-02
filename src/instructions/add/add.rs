use crate::registers::registers::Registers;
use crate::instructions::utils::half_carry_utils::{check_half_carry};

pub fn add(registers: &mut Registers, value: u8) {
    let original_value = registers.a.get();
    let (new_value, overflow) = registers.a.overflowing_add(value);
    
    registers.a.set(new_value);
    registers.zero_flag.set(new_value == 0);
    registers.subtract_flag.set(false);
    registers.carry_flag.set(overflow);
    registers.half_carry_flag.set(check_half_carry(original_value, value));
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
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        
        add(&mut registers, VALUE_TO_ADD);
        
        assert_eq!(as_hex!(registers.a), as_hex!(EXPECTED_VALUE));
    }
    
    #[test]
    fn test_add_becomes_zero_sets_zero_flag_to_true() {
        const INITIAL_A: u8 = 0x12;
        const VALUE_TO_ADD: u8 = (0x00 as u8).overflowing_sub(INITIAL_A).0;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        
        add(&mut registers, VALUE_TO_ADD);
        
        assert_eq!(registers.zero_flag.get(), true);
    }
    
    #[test]
    fn test_add_becomes_non_zero_sets_zero_flag_to_false() {
        const INITIAL_A: u8 = 0x12;
        const VALUE_TO_ADD: u8 = 0xFF - INITIAL_A;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        
        add(&mut registers, VALUE_TO_ADD);
        
        assert_eq!(registers.zero_flag.get(), false);
    }
    
    #[test]
    fn test_add_turns_subtract_flag_off() {
        const INITIAL_A: u8 = 0x12;
        const VALUE_TO_ADD: u8 = 0x34;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.subtract_flag.set(true);
        
        add(&mut registers, VALUE_TO_ADD);
        
        assert_eq!(registers.subtract_flag.get(), false);
    }
    
    #[test]
    fn test_add_no_overflow_sets_carry_flag_off() {
        const INITIAL_A: u8 = 0xFF;
        const VALUE_TO_ADD: u8 = 0x00;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.carry_flag.set(true);
        
        add(&mut registers, VALUE_TO_ADD);
        
        assert_eq!(registers.carry_flag.get(), false);
    }
    
    #[test]
    fn test_add_overflowed_sets_carry_flag_on() {
        const INITIAL_A: u8 = 0xFF;
        const VALUE_TO_ADD: u8 = 0x1;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.carry_flag.set(false);
        
        add(&mut registers, VALUE_TO_ADD);
        
        assert_eq!(registers.carry_flag.get(), true);
    }
    
    #[test]
    fn test_add_no_lower_overflow_sets_half_carry_flag_off() {
        const INITIAL_A: u8 = 0xB0;
        const VALUE_TO_ADD: u8 = 0x10;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.half_carry_flag.set(true);
        
        add(&mut registers, VALUE_TO_ADD);
        
        assert_eq!(registers.half_carry_flag.get(), false);
    }
    
    #[test]
    fn test_add_overflowed_sets_half_carry_flag_on() {
        const INITIAL_A: u8 = 0xFF;
        const VALUE_TO_ADD: u8 = 0x1;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.half_carry_flag.set(false);
        
        add(&mut registers, VALUE_TO_ADD);
        
        assert_eq!(registers.half_carry_flag.get(), true);
    }
}