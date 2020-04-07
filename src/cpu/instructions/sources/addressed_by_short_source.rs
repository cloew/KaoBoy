use super::{ByteSource, ConstantShortSource, DoubleRegisterSource, ShortSource};
use super::super::utils::{PostOpFn, dec_double_register, inc_double_register};
use super::super::super::instruction_context::InstructionContext;
use super::super::super::registers::DoubleRegisterName;
use crate::boxed;

pub struct AddressedByShortSource {
    _address_source: Box<dyn ShortSource>,
    _follow_up_fn: Option<PostOpFn>,
}

impl AddressedByShortSource {
	pub fn new(address_source: Box<dyn ShortSource>) -> AddressedByShortSource {
		return AddressedByShortSource {_address_source: address_source, _follow_up_fn: None};
	}
    
	pub fn new_from_constant() -> AddressedByShortSource {
		return AddressedByShortSource {_address_source: boxed!(ConstantShortSource::new()), _follow_up_fn: None};
	}
    
	pub fn new_from_register(register_name: DoubleRegisterName) -> AddressedByShortSource {
		return AddressedByShortSource {_address_source: boxed!(DoubleRegisterSource::new(register_name)), _follow_up_fn: None};
	}
    
	pub fn new_from_register_then_decrement(register_name: DoubleRegisterName) -> AddressedByShortSource {
		return AddressedByShortSource {_address_source: boxed!(DoubleRegisterSource::new(register_name)), _follow_up_fn: Some(dec_double_register)};
	}
    
	pub fn new_from_register_then_increment(register_name: DoubleRegisterName) -> AddressedByShortSource {
		return AddressedByShortSource {_address_source: boxed!(DoubleRegisterSource::new(register_name)), _follow_up_fn: Some(inc_double_register)};
	}
}

impl ByteSource for AddressedByShortSource {
	fn read(&self, context: &mut InstructionContext) -> u8 {
        let address = self._address_source.read(context);
        
        match self._follow_up_fn {
            Some(follow_up_fn) => (follow_up_fn)(context, DoubleRegisterName::HL), // Hardcode to HL since that's the only register with follow up logic
            None => (),
        }
        
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
    
    #[test]
    fn test_read_with_follow_up_reads_memory() {
        const EXPECTED_ADDRESS: u16 = 0xFEDC;
        const EXPECTED_VALUE: u8 = 0x78;
        let mut context = build_test_instruction_context();
        context.registers_mut().hl.set(EXPECTED_ADDRESS);
        context.memory_mut().write_byte(EXPECTED_ADDRESS, EXPECTED_VALUE);
        let source = AddressedByShortSource::new_from_register_then_decrement(DoubleRegisterName::HL);
        
        let result = source.read(&mut context);
        
        assert_eq!(as_hex!(result), as_hex!(EXPECTED_VALUE));
    }
    
    #[test]
    fn test_read_with_follow_up_fn() {
        const ORIGINAL_ADDRESS: u16 = 0xFEDC;
        const EXPECTED_ADDRESS: u16 = ORIGINAL_ADDRESS-1;
        let mut context = build_test_instruction_context();
        context.registers_mut().hl.set(ORIGINAL_ADDRESS);
        let source = AddressedByShortSource::new_from_register_then_decrement(DoubleRegisterName::HL);
        
        source.read(&mut context);
        
        assert_eq!(as_hex!(context.registers_mut().hl), as_hex!(EXPECTED_ADDRESS));
    }
}
