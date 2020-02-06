use super::source::Source;
use super::super::super::instruction_context::InstructionContext;

pub struct ConstantSource {
}

impl ConstantSource {
	pub fn new() -> ConstantSource {
		return ConstantSource {};
	}
}

impl Source for ConstantSource {
	fn read(&self, context: &InstructionContext) -> u8 {
        return context.program_mut().read_next_byte();
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
        const EXPECTED_VALUE: u8 = 0x12;
        let context = build_test_instruction_context();
        let source = ConstantSource::new();
        
        context.program_mut().set_counter(COUNTER);
        context.program_mut()._memory.set_byte(COUNTER, EXPECTED_VALUE);
        
        let result = source.read(&context);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_VALUE));
    }
}
