use super::destination::Destination;
use super::super::super::instruction_context::InstructionContext;
use super::super::super::registers::register_names::RegisterName;

pub struct RegisterDestination {
    _name: RegisterName,
}

impl RegisterDestination {
	pub fn new(register_name: RegisterName) -> RegisterDestination {
		return RegisterDestination {_name: register_name};
	}
}

impl Destination for RegisterDestination {
	fn assign(&self, context: &mut InstructionContext, new_value: u8) {
        context.registers_mut().get_mut(self._name).set(new_value);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    use crate::cpu::testing::build_test_instruction_context;
    
    #[test]
    fn test_run_returns_register_value() {
        const EXPECTED_A: u8 = 0x12;
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(0x00);
        let source = RegisterDestination::new(RegisterName::A);
        
        source.assign(&mut context, EXPECTED_A);
        
        assert_eq!(as_hex!(context.registers().a), as_hex!(EXPECTED_A));
    }
}
