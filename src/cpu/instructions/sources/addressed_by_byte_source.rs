use super::ByteSource;
use super::super::utils::build_full_address;
use super::super::super::instruction_context::InstructionContext;
use super::super::super::registers::DoubleRegisterName;

pub struct AddressedByByteSource {
    _address_source: Box<dyn ByteSource>,
}

impl AddressedByByteSource {
	pub fn new(address_source: Box<dyn ByteSource>) -> AddressedByByteSource {
		return AddressedByByteSource {_address_source: address_source};
	}
}

impl ByteSource for AddressedByByteSource {
	fn read(&self, context: &InstructionContext) -> u8 {
        let address = self._address_source.read(context);
        return context.memory().read_byte(build_full_address(address));
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::RegisterSource;
    use super::super::super::super::registers::RegisterName;
    use crate::{as_hex, boxed, build_u16};
    use crate::cpu::testing::build_test_instruction_context;
    
    #[test]
    fn test_read_reads_memory() {
        const EXPECTED_VALUE: u8 = 0xAB;
        const SUBADDRESS_VALUE: u8 = 0x78;
        const EXPECTED_ADDRESS: u16 = build_u16!(0xFF, SUBADDRESS_VALUE);
        let mut context = build_test_instruction_context();
        context.registers_mut().a.set(SUBADDRESS_VALUE);
        context.memory_mut().write_byte(EXPECTED_ADDRESS, EXPECTED_VALUE);
        let source = AddressedByByteSource::new(boxed!(RegisterSource::new(RegisterName::A)));
        
        let result = source.read(&mut context);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_VALUE));
    }
}
