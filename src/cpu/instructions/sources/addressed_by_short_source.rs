use super::{ByteSource, ConstantShortSource, DoubleRegisterSource, ShortSource};
use super::super::super::instruction_context::InstructionContext;
use super::super::super::registers::DoubleRegisterName;
use crate::boxed;

pub struct AddressedByShortSource {
    _address_source: Box<dyn ShortSource>,
}

impl AddressedByShortSource {
	pub fn new(address_source: Box<dyn ShortSource>) -> AddressedByShortSource {
		return AddressedByShortSource {_address_source: address_source};
	}
    
	pub fn new_from_constant() -> AddressedByShortSource {
		return AddressedByShortSource {_address_source: boxed!(ConstantShortSource::new())};
	}
    
	pub fn new_from_register(register_name: DoubleRegisterName) -> AddressedByShortSource {
		return AddressedByShortSource {_address_source: boxed!(DoubleRegisterSource::new(register_name))};
	}
}

impl ByteSource for AddressedByShortSource {
	fn read(&self, context: &InstructionContext) -> u8 {
        let address = self._address_source.read(context);
        return context.memory().read_byte(address);
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_hex;
    use crate::cpu::testing::build_test_instruction_context;
    
    #[test]
    fn test_read_reads_memory() {
        const EXPECTED_VALUE: u8 = 0xAB;
        const EXPECTED_ADDRESS: u16 = 0x789A;
        let mut context = build_test_instruction_context();
        context.registers_mut().hl.set(EXPECTED_ADDRESS);
        context.memory_mut().write_byte(EXPECTED_ADDRESS, EXPECTED_VALUE);
        let source = AddressedByShortSource::new_from_register(DoubleRegisterName::HL);
        
        let result = source.read(&mut context);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_VALUE));
    }
}
