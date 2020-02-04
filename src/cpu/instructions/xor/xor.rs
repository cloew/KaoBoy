use super::super::super::registers::registers::Registers;

pub fn xor(registers: &mut Registers, left_value: u8, right_value: u8) -> u8 {
    let new_value = left_value ^ right_value;
    
    registers.zero_flag.set(new_value == 0);
    registers.subtract_flag.reset();
    registers.carry_flag.reset();
    registers.half_carry_flag.reset();
    
    return new_value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    
    #[test]
    fn test_xor_xors_to_a_register() {
        const LEFT_VALUE: u8 = 0x34;
        const RIGHT_VALUE: u8 = 0x12;
        const EXPECTED_VALUE: u8 = LEFT_VALUE ^ RIGHT_VALUE;
        
        let mut registers = Registers::new();
        
        let result = xor(&mut registers, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_VALUE));
    }
    
    #[test]
    fn test_xor_becomes_zero_sets_zero_flag_to_true() {
        const LEFT_VALUE: u8 = 0x12;
        const RIGHT_VALUE: u8 = LEFT_VALUE;
        
        let mut registers = Registers::new();
        registers.zero_flag.reset();
        
        xor(&mut registers, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(registers.zero_flag.get(), true);
    }
    
    #[test]
    fn test_xor_becomes_non_zero_sets_zero_flag_to_false() {
        const LEFT_VALUE: u8 = 0x12;
        const RIGHT_VALUE: u8 = 0xFF;
        
        let mut registers = Registers::new();
        registers.zero_flag.activate();
        
        xor(&mut registers, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(registers.zero_flag.get(), false);
    }
    
    #[test]
    fn test_xor_turns_subtract_flag_off() {
        const LEFT_VALUE: u8 = 0x34;
        const RIGHT_VALUE: u8 = 0x12;
        
        let mut registers = Registers::new();
        registers.subtract_flag.activate();
        
        xor(&mut registers, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(registers.subtract_flag.get(), false);
    }
    
    #[test]
    fn test_xor_turns_carry_flag_off() {
        const LEFT_VALUE: u8 = 0xFF;
        const RIGHT_VALUE: u8 = 0x00;
        
        let mut registers = Registers::new();
        registers.carry_flag.activate();
        
        xor(&mut registers, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(registers.carry_flag.get(), false);
    }
    
    #[test]
    fn test_xor_turns_half_carry_flag_off() {
        const LEFT_VALUE: u8 = 0x20;
        const RIGHT_VALUE: u8 = 0x10;
        
        let mut registers = Registers::new();
        registers.half_carry_flag.activate();
        
        xor(&mut registers, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(registers.half_carry_flag.get(), false);
    }
}