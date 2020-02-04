use super::destination::Destination;
use super::super::super::registers::register_names::RegisterName;
use super::super::super::registers::registers::Registers;

pub struct RegisterDestination {
    _name: RegisterName,
}

impl RegisterDestination {
	pub fn new(register_name: RegisterName) -> RegisterDestination {
		return RegisterDestination {_name: register_name};
	}
}

impl Destination for RegisterDestination {
	fn assign(&self, registers: &mut Registers, new_value: u8) {
        registers.get_mut(self._name).set(new_value);
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
        registers.a.set(0x00);
        let source = RegisterDestination::new(RegisterName::A);
        
        source.assign(&mut registers, EXPECTED_A);
        
        assert_eq!(as_hex!(registers.a), as_hex!(EXPECTED_A));
    }
}
