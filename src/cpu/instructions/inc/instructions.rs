use super::{inc, inc_short};
use super::super::instruction::Instruction;
use super::super::common::{UnaryByteOp, UnaryShortOp};
use super::super::sources::{AddressedByShortSource, DoubleRegisterSource, RegisterSource};
use super::super::destinations::{AddressedByDoubleRegisterDestination, DoubleRegisterDestination, RegisterDestination};
use super::super::super::registers::{DoubleRegisterName, RegisterName};
use crate::{boxed, optional_boxed};

fn build_inc_register(register: RegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryByteOp::new(
            boxed!(RegisterSource::new(register)),
            inc,
            boxed!(RegisterDestination::new(register))
        )
    );
}

fn build_inc_memory_location(register: DoubleRegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryByteOp::new(
            boxed!(AddressedByShortSource::new_from_register(register)),
            inc,
            boxed!(AddressedByDoubleRegisterDestination::new(register))
        )
    );
}

fn build_inc_double_register(register: DoubleRegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryShortOp::new(
            boxed!(DoubleRegisterSource::new(register)),
            inc_short,
            boxed!(DoubleRegisterDestination::new(register))
        )
    );
}

pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        // Increment Registers
        0x04 => build_inc_register(RegisterName::B),
        0x14 => build_inc_register(RegisterName::D),
        0x24 => build_inc_register(RegisterName::H),
        0x34 => build_inc_memory_location(DoubleRegisterName::HL),
        0x0C => build_inc_register(RegisterName::C),
        0x1C => build_inc_register(RegisterName::E),
        0x2C => build_inc_register(RegisterName::L),
        0x3C => build_inc_register(RegisterName::A),
        // Increment Double Registers
        0x03 => build_inc_double_register(DoubleRegisterName::BC),
        0x13 => build_inc_double_register(DoubleRegisterName::DE),
        0x23 => build_inc_double_register(DoubleRegisterName::HL),
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_instruction_returns_instruction() {
        const LOAD_INSTRUCTION: u8 = 0x40;
        
        load_instruction(LOAD_INSTRUCTION);
        
        // Not sure if there'e anything I can assert on
    }
}
