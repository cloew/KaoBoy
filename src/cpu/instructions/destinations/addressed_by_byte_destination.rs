use super::byte_destination::ByteDestination;
use super::super::sources::ByteSource;
use super::super::utils::{PostOpFn, build_full_address, dec_double_register, inc_double_register};
use super::super::super::instruction_context::InstructionContext;
use super::super::super::registers::DoubleRegisterName;
use crate::build_u16;

pub struct AddressedByByteDestination {
    _address_source: Box<dyn ByteSource>,
}

impl AddressedByByteDestination {
	pub fn new(address_source: Box<dyn ByteSource>) -> AddressedByByteDestination {
		return AddressedByByteDestination {_address_source: address_source};
	}
}

impl ByteDestination for AddressedByByteDestination {
	fn assign(&self, context: &mut InstructionContext, new_value: u8) {
        let address = self._address_source.read(context);
        context.memory_mut().write_byte(build_full_address(address), new_value);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::sources::RegisterSource;
    use super::super::super::super::registers::RegisterName;
    use crate::{as_hex, boxed};
    use crate::cpu::testing::build_test_instruction_context;
    
    #[test]
    fn test_assign_assigns_memory() {
        const EXPECTED_VALUE: u8 = 0xAB;
        const SUBADDRESS_VALUE: u8 = 0x78;
        const EXPECTED_ADDRESS: u16 = build_u16!(0xFF, SUBADDRESS_VALUE);
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(SUBADDRESS_VALUE);
        context.memory_mut().write_byte(EXPECTED_ADDRESS, 0x00);
        let destination = AddressedByByteDestination::new(boxed!(RegisterSource::new(RegisterName::A)));
        
        destination.assign(&mut context, EXPECTED_VALUE);
        
        assert_eq!(as_hex!(context.memory().read_byte(EXPECTED_ADDRESS)), as_hex!(EXPECTED_VALUE));
    }
}
