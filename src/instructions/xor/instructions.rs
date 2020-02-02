use crate::instructions::instruction::{Instruction};
use crate::instructions::xor::xor_from_register::{XorFromRegister};
use crate::registers::register_names::{RegisterName};

pub fn load_instruction(instruction_byte: u8) -> Option<Box<dyn Instruction>> {
    return match instruction_byte {
        0xA8 => Some(Box::new(XorFromRegister::new(RegisterName::B))),
        0xA9 => Some(Box::new(XorFromRegister::new(RegisterName::C))),
        0xAA => Some(Box::new(XorFromRegister::new(RegisterName::D))),
        0xAB => Some(Box::new(XorFromRegister::new(RegisterName::E))),
        0xAC => Some(Box::new(XorFromRegister::new(RegisterName::H))),
        0xAD => Some(Box::new(XorFromRegister::new(RegisterName::L))),
        0xAF => Some(Box::new(XorFromRegister::new(RegisterName::A))),
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_instruction_returns_instruction() {
        const XOR_INSTRUCTION: u8 = 0x87;
        
        load_instruction(XOR_INSTRUCTION);
        
        // Not sure if there'e anything I can assert on
    }
}
