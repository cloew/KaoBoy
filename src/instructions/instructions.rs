use crate::instructions::add;
use crate::instructions::instruction::{Instruction};

pub fn load_instruction(instruction_byte: u8) -> Box<dyn Instruction> {
    return add::instructions::load_instruction(instruction_byte);
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
