use super::ShortSource;
use super::super::super::InstructionContext;

pub struct ConstantShortSource {
}

impl ConstantShortSource {
	pub fn new() -> ConstantShortSource {
		return ConstantShortSource {};
	}
}

impl ShortSource for ConstantShortSource {
	fn read(&self, context: &InstructionContext) -> u16 {
        return context.program_mut().read_next_short();
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    use crate::cpu::testing::build_test_instruction_context;
    
    #[test]
    fn test_run_returns_register_value() {
        const COUNTER: u16 = 0xABCD;
        const EXPECTED_VALUE: u16 = 0xFECD;
        let context = build_test_instruction_context();
        let source = ConstantShortSource::new();
        
        context.program_mut().set_counter(COUNTER);
        context.program_mut()._memory.borrow_mut().set_short(COUNTER, EXPECTED_VALUE);
        
        let result = source.read(&context);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_VALUE));
    }
}
