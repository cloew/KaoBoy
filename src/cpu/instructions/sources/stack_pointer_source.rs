use super::ShortSource;
use super::super::super::InstructionContext;

pub struct StackPointerSource {
}

impl StackPointerSource {
	pub fn new() -> StackPointerSource {
		return StackPointerSource {};
	}
}

impl ShortSource for StackPointerSource {
	fn read(&self, context: &mut InstructionContext) -> u16 {
        return context.stack().get_pointer();
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    use crate::cpu::testing::build_test_instruction_context;
    
    #[test]
    fn test_run_returns_register_value() {
        const EXPECTED_VALUE: u16 = 0xFECD;
        let mut context = build_test_instruction_context();
        context.stack_mut().set_pointer(EXPECTED_VALUE);
        let source = StackPointerSource::new();
        
        let result = source.read(&mut context);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_VALUE));
    }
}
