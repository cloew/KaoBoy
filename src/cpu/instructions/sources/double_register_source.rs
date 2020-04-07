use super::ShortSource;
use super::super::super::InstructionContext;
use super::super::super::registers::DoubleRegisterName;

pub struct DoubleRegisterSource {
    _name: DoubleRegisterName,
}

impl DoubleRegisterSource {
	pub fn new(register_name: DoubleRegisterName) -> DoubleRegisterSource {
		return DoubleRegisterSource {_name: register_name};
	}
}

impl ShortSource for DoubleRegisterSource {
	fn read(&self, context: &mut InstructionContext) -> u16 {
        return context.registers().get_double(self._name).get();
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
        context.registers_mut().hl.set(EXPECTED_VALUE);
        let source = DoubleRegisterSource::new(DoubleRegisterName::HL);
        
        let result = source.read(&mut context);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_VALUE));
    }
}
