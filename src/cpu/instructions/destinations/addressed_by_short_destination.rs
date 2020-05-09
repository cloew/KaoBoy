use super::byte_destination::ByteDestination;
use super::super::sources::{ConstantShortSource, DoubleRegisterSource, ShortSource};
use super::super::utils::{PostOpFn, dec_double_register, inc_double_register};
use super::super::super::instruction_context::InstructionContext;
use super::super::super::registers::DoubleRegisterName;
use crate::boxed;

pub struct AddressedByShortDestination {
    _address_source: Box<dyn ShortSource>,
    _follow_up_fn: Option<PostOpFn>,
}

impl AddressedByShortDestination {
	pub fn new(source: Box<dyn ShortSource>) -> AddressedByShortDestination {
		return AddressedByShortDestination {_address_source: source, _follow_up_fn: None};
	}
    
	pub fn new_from_register(register_name: DoubleRegisterName) -> AddressedByShortDestination {
		return AddressedByShortDestination::new(boxed!(DoubleRegisterSource::new(register_name)));
	}
    
	pub fn new_from_register_then_decrement(register_name: DoubleRegisterName) -> AddressedByShortDestination {
		return AddressedByShortDestination {_address_source: boxed!(DoubleRegisterSource::new(register_name)), _follow_up_fn: Some(dec_double_register)};
	}
    
	pub fn new_from_register_then_increment(register_name: DoubleRegisterName) -> AddressedByShortDestination {
		return AddressedByShortDestination {_address_source: boxed!(DoubleRegisterSource::new(register_name)), _follow_up_fn: Some(inc_double_register)};
	}
}

impl ByteDestination for AddressedByShortDestination {
	fn assign(&self, context: &mut InstructionContext, new_value: u8) {
        let address = self._address_source.read(context);
        context.memory_mut().write_byte(address, new_value);
        
        match self._follow_up_fn {
            Some(follow_up_fn) => (follow_up_fn)(context, DoubleRegisterName::HL), // Hardcode to HL since that's the only register with follow up logic
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
        let destination = AddressedByShortDestination::new_from_register(DoubleRegisterName::HL);
        
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
        let destination = AddressedByShortDestination::new_from_register_then_decrement(DoubleRegisterName::HL);
        
        destination.assign(&mut context, EXPECTED_VALUE);
        
        assert_eq!(as_hex!(context.memory().read_byte(EXPECTED_ADDRESS)), as_hex!(EXPECTED_VALUE));
    }
    
    #[test]
    fn test_assign_with_follow_up_fn() {
        const ORIGINAL_ADDRESS: u16 = 0xFEDC;
        const EXPECTED_ADDRESS: u16 = ORIGINAL_ADDRESS-1;
        let mut context = build_test_instruction_context();
        context.registers_mut().hl.set(ORIGINAL_ADDRESS);
        let destination = AddressedByShortDestination::new_from_register_then_decrement(DoubleRegisterName::HL);
        
        destination.assign(&mut context, 0x78);
        
        assert_eq!(as_hex!(context.registers_mut().hl), as_hex!(EXPECTED_ADDRESS));
    }
}
