use super::source::Source;
use super::super::super::registers::register_names::RegisterName;
use super::super::super::registers::registers::Registers;

pub struct RegisterSource {
    _name: RegisterName,
}

impl RegisterSource {
	pub fn new(register_name: RegisterName) -> RegisterSource {
		return RegisterSource {_name: register_name};
	}
}

impl Source for RegisterSource {
	fn read(&self, registers: &Registers) -> u8 {
        return registers.get(self._name).get();
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    
    #[test]
    fn test_run_returns_register_value() {
        const EXPECTED_A: u8 = 0x12;
        let mut registers = Registers::new();
        registers.a.set(EXPECTED_A);
        let source = RegisterSource::new(RegisterName::A);
        
        let result = source.read(&registers);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_A));
    }
}
