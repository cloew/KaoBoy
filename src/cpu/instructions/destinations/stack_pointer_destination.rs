use super::short_destination::ShortDestination;
use super::super::super::InstructionContext;

pub struct StackPointerDestination {
}

impl StackPointerDestination {
	pub fn new() -> StackPointerDestination {
		return StackPointerDestination {};
	}
}

impl ShortDestination for RegisterDestination {
	fn assign(&self, context: &mut InstructionContext, new_value: u16) {
        context.stack_mut().set_pointer(new_value);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    use crate::cpu::testing::build_test_instruction_context;
    
    #[test]
    fn test_run_returns_register_value() {
        const EXPECTED_POINTER: u8 = 0x12;
        let mut context = build_test_instruction_context();
        context.stack_mut().set_pointer(0x00);
        let source = StackPointerDestination::new();
        
        source.assign(&mut context, EXPECTED_POINTER);
        
        assert_eq!(as_hex!(context.stack_mut()._pointer), as_hex!(EXPECTED_POINTER));
    }
}
