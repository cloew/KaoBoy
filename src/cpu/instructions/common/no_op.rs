use super::super::super::instruction_context::InstructionContext;

pub fn byte_no_op(_context: &mut InstructionContext, value: u8) -> u8 {
    return value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    use crate::cpu::testing::build_test_instruction_context;
    
    #[test]
    fn test_run_calls_source_op_and_destination() {
        const VALUE: u8 = 0x12;
        let mut context = build_test_instruction_context();
        
        let result = byte_no_op(&mut context, VALUE);
        
        assert_eq!(as_hex!(result), as_hex!(VALUE));
    }
}
