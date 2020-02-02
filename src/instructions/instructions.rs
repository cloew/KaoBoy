use crate::instructions::add;
use crate::instructions::instruction::{Instruction};
use crate::{as_hex};

pub fn load_instruction(instruction_byte: u8) -> Box<dyn Instruction> {
    let optional_instruction = add::instructions::load_instruction(instruction_byte);
    return match optional_instruction {
        Some(instruction) => instruction,
        None => panic!("Unknown instruction: {}", as_hex!(instruction_byte)),
    }
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
