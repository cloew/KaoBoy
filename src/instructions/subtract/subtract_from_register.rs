use crate::instructions::subtract::subtract::{subtract};
use crate::instructions::instruction::{Instruction};
use crate::registers::register_names::{RegisterName};
use crate::registers::registers::{Registers};

pub struct SubtractFromRegister {
    _name: RegisterName,
}

impl SubtractFromRegister {
	pub fn new(register_name: RegisterName) -> SubtractFromRegister {
		return SubtractFromRegister {_name: register_name};
	}
}

impl Instruction for SubtractFromRegister {
	fn run(&self, registers: &mut Registers) {
        let value_to_subtract = registers.get(self._name).get();
        subtract(registers, value_to_subtract);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    
    #[test]
    fn test_run_subtracts_value() {
        const INITIAL_A: u8 = 0x12;
        const INITIAL_B: u8 = 0x03;
        const EXPECTED_A: u8 = INITIAL_A - INITIAL_B;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.b.set(INITIAL_B);
        
        let instruction = SubtractFromRegister::new(RegisterName::B);
        
        instruction.run(&mut registers);
        
        assert_eq!(as_hex!(registers.a), as_hex!(EXPECTED_A));
    }
}
