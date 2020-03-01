use super::byte_destination::ByteDestination;
use super::super::super::instruction_context::InstructionContext;
use super::super::super::registers::DoubleRegisterName;

pub struct AddressedByDoubleRegisterDestination {
    _name: DoubleRegisterName,
}

impl AddressedByDoubleRegisterDestination {
	pub fn new(register_name: DoubleRegisterName) -> AddressedByDoubleRegisterDestination {
		return AddressedByDoubleRegisterDestination {_name: register_name};
	}
}

impl ByteDestination for AddressedByDoubleRegisterDestination {
	fn assign(&self, context: &mut InstructionContext, new_value: u8) {
        let address = context.registers_mut().get_double(self._name).get();
        context.memory_mut().write_byte(address, new_value);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    use crate::cpu::testing::build_test_instruction_context;
    use crate::{get_lower_u8, get_upper_u8};
    
    #[test]
    fn test_run_assigns_memory() {
        const EXPECTED_ADDRESS: u16 = 0xFEDC;
        const EXPECTED_VALUE: u8 = 0x78;
        let mut context = build_test_instruction_context();
        context.registers_mut().h.set(get_upper_u8!(EXPECTED_ADDRESS));
        context.registers_mut().l.set(get_lower_u8!(EXPECTED_ADDRESS));
        context.memory_mut().write_byte(EXPECTED_ADDRESS, 0x00);
        let destination = AddressedByDoubleRegisterDestination::new(DoubleRegisterName::HL);
        
        destination.assign(&mut context, EXPECTED_VALUE);
        
        assert_eq!(as_hex!(context.memory().read_byte(EXPECTED_ADDRESS)), as_hex!(EXPECTED_VALUE));
    }
}
