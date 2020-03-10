use super::super::super::InstructionContext;
use super::super::super::registers::DoubleRegisterName;

pub type PostOpFn = fn(&mut InstructionContext, DoubleRegisterName);

pub fn inc_double_register(context: &mut InstructionContext, register_name: DoubleRegisterName) {
    context.registers_mut().get_double_mut(register_name).increment();
}

pub fn dec_double_register(context: &mut InstructionContext, register_name: DoubleRegisterName) {
    context.registers_mut().get_double_mut(register_name).decrement();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    use crate::cpu::testing::build_test_instruction_context;
    
    #[test]
    fn test_inc_double_register_increments_register() {
        const ORIGINAL_VALUE: u16 = 0xFEDC;
        const EXPECTED_VALUE: u16 = ORIGINAL_VALUE + 1;
        let mut context = build_test_instruction_context();
        context.registers_mut().hl.set(ORIGINAL_VALUE);
        
        inc_double_register(&mut context, DoubleRegisterName::HL);
        
        assert_eq!(as_hex!(context.registers().get_double(DoubleRegisterName::HL)), as_hex!(EXPECTED_VALUE));
    }
    
    #[test]
    fn test_dec_double_register_decrements_register() {
        const ORIGINAL_VALUE: u16 = 0xFEDC;
        const EXPECTED_VALUE: u16 = ORIGINAL_VALUE - 1;
        let mut context = build_test_instruction_context();
        context.registers_mut().hl.set(ORIGINAL_VALUE);
        
        dec_double_register(&mut context, DoubleRegisterName::HL);
        
        assert_eq!(as_hex!(context.registers().get_double(DoubleRegisterName::HL)), as_hex!(EXPECTED_VALUE));
    }
}
