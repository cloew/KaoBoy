use crate::registers::registers::Registers;

pub fn xor(registers: &mut Registers, value: u8) {
    let original_value = registers.a.get();
    let new_value = original_value ^ value;
    
    registers.a.set(new_value);
    registers.zero_flag.set(new_value == 0);
    registers.subtract_flag.reset();
    registers.carry_flag.reset();
    registers.half_carry_flag.reset();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    
    #[test]
    fn test_xor_xors_to_a_register() {
        const INITIAL_A: u8 = 0x34;
        const VALUE_TO_XOR: u8 = 0x12;
        const EXPECTED_VALUE: u8 = INITIAL_A ^ VALUE_TO_XOR;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        
        xor(&mut registers, VALUE_TO_XOR);
        
        assert_eq!(as_hex!(registers.a), as_hex!(EXPECTED_VALUE));
    }
    
    #[test]
    fn test_xor_becomes_zero_sets_zero_flag_to_true() {
        const INITIAL_A: u8 = 0x12;
        const VALUE_TO_XOR: u8 = INITIAL_A;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.zero_flag.set(false);
        
        xor(&mut registers, VALUE_TO_XOR);
        
        assert_eq!(registers.zero_flag.get(), true);
    }
    
    #[test]
    fn test_xor_becomes_non_zero_sets_zero_flag_to_false() {
        const INITIAL_A: u8 = 0x12;
        const VALUE_TO_XOR: u8 = 0xFF;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.zero_flag.set(true);
        
        xor(&mut registers, VALUE_TO_XOR);
        
        assert_eq!(registers.zero_flag.get(), false);
    }
    
    #[test]
    fn test_xor_turns_subtract_flag_off() {
        const INITIAL_A: u8 = 0x34;
        const VALUE_TO_XOR: u8 = 0x12;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.subtract_flag.set(true);
        
        xor(&mut registers, VALUE_TO_XOR);
        
        assert_eq!(registers.subtract_flag.get(), false);
    }
    
    #[test]
    fn test_xor_turns_carry_flag_off() {
        const INITIAL_A: u8 = 0xFF;
        const VALUE_TO_XOR: u8 = 0x00;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.carry_flag.set(true);
        
        xor(&mut registers, VALUE_TO_XOR);
        
        assert_eq!(registers.carry_flag.get(), false);
    }
    
    #[test]
    fn test_xor_turns_half_carry_flag_off() {
        const INITIAL_A: u8 = 0x20;
        const VALUE_TO_XOR: u8 = 0x10;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.half_carry_flag.set(true);
        
        xor(&mut registers, VALUE_TO_XOR);
        
        assert_eq!(registers.half_carry_flag.get(), false);
    }
}