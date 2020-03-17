use super::bit_instruction::BitInstruction;
use super::super::instruction::Instruction;
use super::super::super::registers::register_names::RegisterName;
use super::super::sources::{ByteSource, };
use crate::{boxed, optional_boxed};


fn build_register_bit_instruction(register_name: RegisterName, bit_number: u8) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(BitInstruction::new_for_register(register_name, 0));
}

fn build_bit_instruction(source: Box<dyn ByteSource>, bit_number: u8) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(BitInstruction::new(source, 0));
}


pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        // B Register
        0x40 => build_register_bit_instruction(RegisterName::B, 0),
        0x50 => build_register_bit_instruction(RegisterName::B, 2),
        0x60 => build_register_bit_instruction(RegisterName::B, 4),
        0x70 => build_register_bit_instruction(RegisterName::B, 6),
        0x48 => build_register_bit_instruction(RegisterName::B, 1),
        0x58 => build_register_bit_instruction(RegisterName::B, 3),
        0x68 => build_register_bit_instruction(RegisterName::B, 5),
        0x78 => build_register_bit_instruction(RegisterName::B, 7),
        // C Register
        0x41 => build_register_bit_instruction(RegisterName::C, 0),
        0x51 => build_register_bit_instruction(RegisterName::C, 2),
        0x61 => build_register_bit_instruction(RegisterName::C, 4),
        0x71 => build_register_bit_instruction(RegisterName::C, 6),
        0x49 => build_register_bit_instruction(RegisterName::C, 1),
        0x59 => build_register_bit_instruction(RegisterName::C, 3),
        0x69 => build_register_bit_instruction(RegisterName::C, 5),
        0x79 => build_register_bit_instruction(RegisterName::C, 7),
        // D Register
        0x42 => build_register_bit_instruction(RegisterName::D, 0),
        0x52 => build_register_bit_instruction(RegisterName::D, 2),
        0x62 => build_register_bit_instruction(RegisterName::D, 4),
        0x72 => build_register_bit_instruction(RegisterName::D, 6),
        0x4A => build_register_bit_instruction(RegisterName::D, 1),
        0x5A => build_register_bit_instruction(RegisterName::D, 3),
        0x6A => build_register_bit_instruction(RegisterName::D, 5),
        0x7A => build_register_bit_instruction(RegisterName::D, 7),
        // E Register
        0x43 => build_register_bit_instruction(RegisterName::E, 0),
        0x53 => build_register_bit_instruction(RegisterName::E, 2),
        0x63 => build_register_bit_instruction(RegisterName::E, 4),
        0x73 => build_register_bit_instruction(RegisterName::E, 6),
        0x4B => build_register_bit_instruction(RegisterName::E, 1),
        0x5B => build_register_bit_instruction(RegisterName::E, 3),
        0x6B => build_register_bit_instruction(RegisterName::E, 5),
        0x7B => build_register_bit_instruction(RegisterName::E, 7),
        // H Register
        0x44 => build_register_bit_instruction(RegisterName::H, 0),
        0x54 => build_register_bit_instruction(RegisterName::H, 2),
        0x64 => build_register_bit_instruction(RegisterName::H, 4),
        0x74 => build_register_bit_instruction(RegisterName::H, 6),
        0x4C => build_register_bit_instruction(RegisterName::H, 1),
        0x5C => build_register_bit_instruction(RegisterName::H, 3),
        0x6C => build_register_bit_instruction(RegisterName::H, 5),
        0x7C => build_register_bit_instruction(RegisterName::H, 7),
        // L Register
        0x45 => build_register_bit_instruction(RegisterName::L, 0),
        0x55 => build_register_bit_instruction(RegisterName::L, 2),
        0x65 => build_register_bit_instruction(RegisterName::L, 4),
        0x75 => build_register_bit_instruction(RegisterName::L, 6),
        0x4D => build_register_bit_instruction(RegisterName::L, 1),
        0x5D => build_register_bit_instruction(RegisterName::L, 3),
        0x6D => build_register_bit_instruction(RegisterName::L, 5),
        0x7D => build_register_bit_instruction(RegisterName::L, 7),
        // (HL) Register
        //0x46 => build_bit_instruction(RegisterName::L, 0),
        //0x56 => build_bit_instruction(RegisterName::L, 2),
        //0x66 => build_bit_instruction(RegisterName::L, 4),
        //0x76 => build_bit_instruction(RegisterName::L, 6),
        //0x4E => build_bit_instruction(RegisterName::L, 1),
        //0x5E => build_bit_instruction(RegisterName::L, 3),
        //0x6E => build_bit_instruction(RegisterName::L, 5),
        //0x7E => build_bit_instruction(RegisterName::L, 7),
        // A Register
        0x47 => build_register_bit_instruction(RegisterName::A, 0),
        0x57 => build_register_bit_instruction(RegisterName::A, 2),
        0x67 => build_register_bit_instruction(RegisterName::A, 4),
        0x77 => build_register_bit_instruction(RegisterName::A, 6),
        0x4F => build_register_bit_instruction(RegisterName::A, 1),
        0x5F => build_register_bit_instruction(RegisterName::A, 3),
        0x6F => build_register_bit_instruction(RegisterName::A, 5),
        0x7F => build_register_bit_instruction(RegisterName::A, 7),
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_instruction_returns_instruction() {
        const ADD_INSTRUCTION: u8 = 0x87;
        
        load_instruction(ADD_INSTRUCTION);
        
        // Not sure if there'e anything I can assert on
    }
}
