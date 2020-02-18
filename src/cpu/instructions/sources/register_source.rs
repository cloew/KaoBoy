use super::ByteSource;
use super::super::super::instruction_context::InstructionContext;
use super::super::super::registers::register_names::RegisterName;

pub struct RegisterSource {
    _name: RegisterName,
}

impl RegisterSource {
	pub fn new(register_name: RegisterName) -> RegisterSource {
		return RegisterSource {_name: register_name};
	}
}

impl ByteSource for RegisterSource {
	fn read(&self, context: &InstructionContext) -> u8 {
        return context.registers().get(self._name).get();
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
        let context = build_test_instruction_context();
        context.registers_mut().a.set(EXPECTED_A);
        let source = RegisterSource::new(RegisterName::A);
        
        let result = source.read(&context);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_A));
    }
}
