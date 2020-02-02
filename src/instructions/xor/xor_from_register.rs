use crate::instructions::xor::xor::{xor};
use crate::instructions::instruction::{Instruction};
use crate::registers::register_names::{RegisterName};
use crate::registers::registers::{Registers};

pub struct XorFromRegister {
    _name: RegisterName,
}

impl XorFromRegister {
	pub fn new(register_name: RegisterName) -> XorFromRegister {
		return XorFromRegister {_name: register_name};
	}
}

impl Instruction for XorFromRegister {
	fn run(&self, registers: &mut Registers) {
        let value_to_xor = registers.get(self._name).get();
        xor(registers, value_to_xor);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    
    #[test]
    fn test_run_xors_value() {
        const INITIAL_A: u8 = 0x12;
        const INITIAL_B: u8 = 0x03;
        const EXPECTED_A: u8 = INITIAL_A ^ INITIAL_B;
        
        let mut registers = Registers::new();
		registers.a.set(INITIAL_A);
        registers.b.set(INITIAL_B);
        
        let instruction = XorFromRegister::new(RegisterName::B);
        
        instruction.run(&mut registers);
        
        assert_eq!(as_hex!(registers.a), as_hex!(EXPECTED_A));
    }
}
