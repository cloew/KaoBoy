use super::super::instruction::Instruction;
use super::super::common::no_op::no_op;
use super::super::common::unary_byte_op::UnaryByteOp;
use super::super::destinations::register_destination::RegisterDestination;
use super::super::sources::register_source::RegisterSource;
use super::super::super::registers::register_names::RegisterName;
use crate::{boxed, optional_boxed};

fn build_load_instruction(source_name: RegisterName, destination_name: RegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryByteOp::new(
            boxed!(RegisterSource::new(source_name)),
            no_op,
            boxed!(RegisterDestination::new(destination_name))
        )
    );
}

pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        // Load into Register B
        0x40 => build_load_instruction(RegisterName::B, RegisterName::B),
        0x41 => build_load_instruction(RegisterName::C, RegisterName::B),
        0x42 => build_load_instruction(RegisterName::D, RegisterName::B),
        0x43 => build_load_instruction(RegisterName::E, RegisterName::B),
        0x44 => build_load_instruction(RegisterName::H, RegisterName::B),
        0x45 => build_load_instruction(RegisterName::L, RegisterName::B),
        0x47 => build_load_instruction(RegisterName::A, RegisterName::B),
        // Load into Register C
        0x48 => build_load_instruction(RegisterName::B, RegisterName::C),
        0x49 => build_load_instruction(RegisterName::C, RegisterName::C),
        0x4A => build_load_instruction(RegisterName::D, RegisterName::C),
        0x4B => build_load_instruction(RegisterName::E, RegisterName::C),
        0x4C => build_load_instruction(RegisterName::H, RegisterName::C),
        0x4D => build_load_instruction(RegisterName::L, RegisterName::C),
        0x4F => build_load_instruction(RegisterName::A, RegisterName::C),
        // Load into Register D
        0x50 => build_load_instruction(RegisterName::B, RegisterName::D),
        0x51 => build_load_instruction(RegisterName::C, RegisterName::D),
        0x52 => build_load_instruction(RegisterName::D, RegisterName::D),
        0x53 => build_load_instruction(RegisterName::E, RegisterName::D),
        0x54 => build_load_instruction(RegisterName::H, RegisterName::D),
        0x55 => build_load_instruction(RegisterName::L, RegisterName::D),
        0x57 => build_load_instruction(RegisterName::A, RegisterName::D),
        // Load into Register E
        0x58 => build_load_instruction(RegisterName::B, RegisterName::E),
        0x59 => build_load_instruction(RegisterName::C, RegisterName::E),
        0x5A => build_load_instruction(RegisterName::D, RegisterName::E),
        0x5B => build_load_instruction(RegisterName::E, RegisterName::E),
        0x5C => build_load_instruction(RegisterName::H, RegisterName::E),
        0x5D => build_load_instruction(RegisterName::L, RegisterName::E),
        0x5F => build_load_instruction(RegisterName::A, RegisterName::E),
        // Load into Register H
        0x60 => build_load_instruction(RegisterName::B, RegisterName::H),
        0x61 => build_load_instruction(RegisterName::C, RegisterName::H),
        0x62 => build_load_instruction(RegisterName::D, RegisterName::H),
        0x63 => build_load_instruction(RegisterName::E, RegisterName::H),
        0x64 => build_load_instruction(RegisterName::H, RegisterName::H),
        0x65 => build_load_instruction(RegisterName::L, RegisterName::H),
        0x67 => build_load_instruction(RegisterName::A, RegisterName::H),
        // Load into Register L
        0x68 => build_load_instruction(RegisterName::B, RegisterName::L),
        0x69 => build_load_instruction(RegisterName::C, RegisterName::L),
        0x6A => build_load_instruction(RegisterName::D, RegisterName::L),
        0x6B => build_load_instruction(RegisterName::E, RegisterName::L),
        0x6C => build_load_instruction(RegisterName::H, RegisterName::L),
        0x6D => build_load_instruction(RegisterName::L, RegisterName::L),
        0x6F => build_load_instruction(RegisterName::A, RegisterName::L),
        // Load into Register A
        0x78 => build_load_instruction(RegisterName::B, RegisterName::A),
        0x79 => build_load_instruction(RegisterName::C, RegisterName::A),
        0x7A => build_load_instruction(RegisterName::D, RegisterName::A),
        0x7B => build_load_instruction(RegisterName::E, RegisterName::A),
        0x7C => build_load_instruction(RegisterName::H, RegisterName::A),
        0x7D => build_load_instruction(RegisterName::L, RegisterName::A),
        0x7F => build_load_instruction(RegisterName::A, RegisterName::A),
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
