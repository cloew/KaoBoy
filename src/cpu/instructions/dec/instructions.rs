use super::{dec, dec_short};
use super::super::instruction::Instruction;
use super::super::common::{UnaryByteOp, UnaryShortOp};
use super::super::sources::{AddressedByShortSource, DoubleRegisterSource, RegisterSource, StackPointerSource};
use super::super::destinations::{AddressedByShortDestination, DoubleRegisterDestination, RegisterDestination, StackPointerDestination};
use super::super::super::registers::{DoubleRegisterName, RegisterName};
use crate::{boxed, optional_boxed};

fn build_dec_register(register: RegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryByteOp::new(
            boxed!(RegisterSource::new(register)),
            dec,
            boxed!(RegisterDestination::new(register))
        )
    );
}

fn build_dec_memory_location(register: DoubleRegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryByteOp::new(
            boxed!(AddressedByShortSource::new_from_register(register)),
            dec,
            boxed!(AddressedByShortDestination::new_from_register(register))
        )
    );
}

fn build_dec_double_register(register: DoubleRegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryShortOp::new(
            boxed!(DoubleRegisterSource::new(register)),
            dec_short,
            boxed!(DoubleRegisterDestination::new(register))
        )
    );
}

fn build_dec_stack_pointer() -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryShortOp::new(
            boxed!(StackPointerSource::new()),
            dec_short,
            boxed!(StackPointerDestination::new())
        )
    );
}

pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        // Decrement Registers
        0x05 => build_dec_register(RegisterName::B),
        0x15 => build_dec_register(RegisterName::D),
        0x25 => build_dec_register(RegisterName::H),
        0x35 => build_dec_memory_location(DoubleRegisterName::HL),
        0x0D => build_dec_register(RegisterName::C),
        0x1D => build_dec_register(RegisterName::E),
        0x2D => build_dec_register(RegisterName::L),
        0x3D => build_dec_register(RegisterName::A),
        // Decrement Double Registers
        0x0B => build_dec_double_register(DoubleRegisterName::BC),
        0x1B => build_dec_double_register(DoubleRegisterName::DE),
        0x2B => build_dec_double_register(DoubleRegisterName::HL),
        0x3B => build_dec_stack_pointer(),
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_instruction_returns_instruction() {
        const DEC_INSTRUCTION: u8 = 0x05;
        
        load_instruction(DEC_INSTRUCTION);
        
        // Not sure if there'e anything I can assert on
    }
}
