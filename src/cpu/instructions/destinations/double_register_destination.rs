use super::short_destination::ShortDestination;
use super::super::super::instruction_context::InstructionContext;
use super::super::super::registers::DoubleRegisterName;

pub struct DoubleRegisterDestination {
    _name: DoubleRegisterName,
}

impl DoubleRegisterDestination {
	pub fn new(register_name: DoubleRegisterName) -> DoubleRegisterDestination {
		return DoubleRegisterDestination {_name: register_name};
	}
}

impl ShortDestination for DoubleRegisterDestination {
	fn assign(&self, context: &mut InstructionContext, new_value: u16) {
        context.registers_mut().get_double_mut(self._name).set(new_value);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    use crate::cpu::testing::build_test_instruction_context;
    
    #[test]
    fn test_run_returns_register_value() {
        const EXPECTED_HL: u16 = 0x1234;
        let mut context = build_test_instruction_context();
        context.registers_mut().hl.set(0x0000);
        let destination = DoubleRegisterDestination::new(DoubleRegisterName::HL);
        
        destination.assign(&mut context, EXPECTED_HL);
        
        assert_eq!(as_hex!(context.registers().hl), as_hex!(EXPECTED_HL));
    }
}
