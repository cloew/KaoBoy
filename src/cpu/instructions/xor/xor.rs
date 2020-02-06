use super::super::super::instruction_context::InstructionContext;

pub fn xor(context: &mut InstructionContext, left_value: u8, right_value: u8) -> u8 {
    let new_value = left_value ^ right_value;
    
    context.registers_mut().zero_flag.set(new_value == 0);
    context.registers_mut().subtract_flag.reset();
    context.registers_mut().carry_flag.reset();
    context.registers_mut().half_carry_flag.reset();
    
    return new_value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    use crate::cpu::testing::build_test_instruction_context;
    
    #[test]
    fn test_xor_xors_to_a_register() {
        const LEFT_VALUE: u8 = 0x34;
        const RIGHT_VALUE: u8 = 0x12;
        const EXPECTED_VALUE: u8 = LEFT_VALUE ^ RIGHT_VALUE;
        
        let mut context = build_test_instruction_context();
        
        let result = xor(&mut context, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_VALUE));
    }
    
    #[test]
    fn test_xor_becomes_zero_sets_zero_flag_to_true() {
        const LEFT_VALUE: u8 = 0x12;
        const RIGHT_VALUE: u8 = LEFT_VALUE;
        
        let mut context = build_test_instruction_context();
        context.registers_mut().zero_flag.reset();
        
        xor(&mut context, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(context.registers().zero_flag.get(), true);
    }
    
    #[test]
    fn test_xor_becomes_non_zero_sets_zero_flag_to_false() {
        const LEFT_VALUE: u8 = 0x12;
        const RIGHT_VALUE: u8 = 0xFF;
        
        let mut context = build_test_instruction_context();
        context.registers_mut().zero_flag.activate();
        
        xor(&mut context, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(context.registers().zero_flag.get(), false);
    }
    
    #[test]
    fn test_xor_turns_subtract_flag_off() {
        const LEFT_VALUE: u8 = 0x34;
        const RIGHT_VALUE: u8 = 0x12;
        
        let mut context = build_test_instruction_context();
        context.registers_mut().subtract_flag.activate();
        
        xor(&mut context, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(context.registers().subtract_flag.get(), false);
    }
    
    #[test]
    fn test_xor_turns_carry_flag_off() {
        const LEFT_VALUE: u8 = 0xFF;
        const RIGHT_VALUE: u8 = 0x00;
        
        let mut context = build_test_instruction_context();
        context.registers_mut().carry_flag.activate();
        
        xor(&mut context, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(context.registers().carry_flag.get(), false);
    }
    
    #[test]
    fn test_xor_turns_half_carry_flag_off() {
        const LEFT_VALUE: u8 = 0x20;
        const RIGHT_VALUE: u8 = 0x10;
        
        let mut context = build_test_instruction_context();
        context.registers_mut().half_carry_flag.activate();
        
        xor(&mut context, LEFT_VALUE, RIGHT_VALUE);
        
        assert_eq!(context.registers().half_carry_flag.get(), false);
    }
}