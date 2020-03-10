use super::byte_destination::ByteDestination;
use super::super::utils::{PostOpFn, dec_double_register, inc_double_register};
use super::super::super::instruction_context::InstructionContext;
use super::super::super::registers::DoubleRegisterName;

pub struct AddressedByDoubleRegisterDestination {
    _name: DoubleRegisterName,
    _follow_up_fn: Option<PostOpFn>,
}

impl AddressedByDoubleRegisterDestination {
	pub fn new(register_name: DoubleRegisterName) -> AddressedByDoubleRegisterDestination {
		return AddressedByDoubleRegisterDestination {_name: register_name, _follow_up_fn: None};
	}
    
	pub fn new_assign_then_decrement(register_name: DoubleRegisterName) -> AddressedByDoubleRegisterDestination {
		return AddressedByDoubleRegisterDestination {_name: register_name, _follow_up_fn: Some(dec_double_register)};
	}
    
	pub fn new_assign_then_increment(register_name: DoubleRegisterName) -> AddressedByDoubleRegisterDestination {
		return AddressedByDoubleRegisterDestination {_name: register_name, _follow_up_fn: Some(inc_double_register)};
	}
}

impl ByteDestination for AddressedByDoubleRegisterDestination {
	fn assign(&self, context: &mut InstructionContext, new_value: u8) {
        let address = context.registers_mut().get_double(self._name).get();
        context.memory_mut().write_byte(address, new_value);
        
        match self._follow_up_fn {
            Some(follow_up_fn) => (follow_up_fn)(context, self._name),
            None => (),
        }
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    use crate::cpu::testing::build_test_instruction_context;
    
    #[test]
    fn test_assign_assigns_memory() {
        const EXPECTED_ADDRESS: u16 = 0xFEDC;
        const EXPECTED_VALUE: u8 = 0x78;
        let mut context = build_test_instruction_context();
        context.registers_mut().hl.set(EXPECTED_ADDRESS);
        context.memory_mut().write_byte(EXPECTED_ADDRESS, 0x00);
        let destination = AddressedByDoubleRegisterDestination::new(DoubleRegisterName::HL);
        
        destination.assign(&mut context, EXPECTED_VALUE);
        
        assert_eq!(as_hex!(context.memory().read_byte(EXPECTED_ADDRESS)), as_hex!(EXPECTED_VALUE));
    }
    
    #[test]
    fn test_assign_with_follow_up_assigns_memory() {
        const EXPECTED_ADDRESS: u16 = 0xFEDC;
        const EXPECTED_VALUE: u8 = 0x78;
        let mut context = build_test_instruction_context();
        context.registers_mut().hl.set(EXPECTED_ADDRESS);
        context.memory_mut().write_byte(EXPECTED_ADDRESS, 0x00);
        let destination = AddressedByDoubleRegisterDestination::new_assign_then_decrement(DoubleRegisterName::HL);
        
        destination.assign(&mut context, EXPECTED_VALUE);
        
        assert_eq!(as_hex!(context.memory().read_byte(EXPECTED_ADDRESS)), as_hex!(EXPECTED_VALUE));
    }
    
    #[test]
    fn test_assign_with_follow_up_fn() {
        const ORIGINAL_ADDRESS: u16 = 0xFEDC;
        const EXPECTED_ADDRESS: u16 = ORIGINAL_ADDRESS-1;
        let mut context = build_test_instruction_context();
        context.registers_mut().hl.set(ORIGINAL_ADDRESS);
        let destination = AddressedByDoubleRegisterDestination::new_assign_then_decrement(DoubleRegisterName::HL);
        
        destination.assign(&mut context, 0x78);
        
        assert_eq!(as_hex!(context.registers_mut().hl), as_hex!(EXPECTED_ADDRESS));
    }
}
