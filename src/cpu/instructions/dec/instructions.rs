use super::{dec, dec_short};
use super::super::instruction::Instruction;
use super::super::common::{UnaryByteOp, UnaryShortOp};
use super::super::sources::{DoubleRegisterSource, RegisterSource};
use super::super::destinations::{DoubleRegisterDestination, RegisterDestination};
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

fn build_dec_double_register(register: DoubleRegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryShortOp::new(
            boxed!(DoubleRegisterSource::new(register)),
            dec_short,
            boxed!(DoubleRegisterDestination::new(register))
        )
    );
}

pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        // Decrement Registers
        0x05 => build_dec_register(RegisterName::B),
        0x15 => build_dec_register(RegisterName::D),
        0x25 => build_dec_register(RegisterName::H),
        0x0D => build_dec_register(RegisterName::C),
        0x1D => build_dec_register(RegisterName::E),
        0x2D => build_dec_register(RegisterName::L),
        0x3D => build_dec_register(RegisterName::A),
        // Decrement Double Registers
        0x08 => build_dec_double_register(DoubleRegisterName::BC),
        0x18 => build_dec_double_register(DoubleRegisterName::DE),
        0x28 => build_dec_double_register(DoubleRegisterName::HL),
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
