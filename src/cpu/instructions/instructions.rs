use super::add;
use super::bit;
use super::inc;
use super::jump;
use super::load;
use super::subtract;
use super::xor;
use super::instruction::Instruction;
use super::super::ProgramCounter;
use crate::as_hex;
use std::boxed::Box;
use std::option::Option;

type PackageInstructionLoader = fn(u8) -> Option<Box<(dyn Instruction + 'static)>>;

static PREFIX_INSTRUCTION: u8 = 0xCB;

pub fn load_instruction(program: &mut ProgramCounter) -> Box<dyn Instruction> {
    
    let instruction_byte = program.read_next_byte();
    
    if instruction_byte == PREFIX_INSTRUCTION {
        let instruction_byte = program.read_next_byte();
        return load_prefix_instruction(instruction_byte);
    } else {
        return load_standard_instruction(instruction_byte);
    }
}

fn load_standard_instruction(instruction_byte: u8) -> Box<dyn Instruction> {
    let package_instruction_loaders: Vec<PackageInstructionLoader>
            = vec![
        add::instructions::load_instruction,
        inc::instructions::load_instruction,
        jump::instructions::load_instruction,
        load::instructions::load_instruction,
        subtract::instructions::load_instruction,
        xor::instructions::load_instruction,
    ];
    
    let next_instruction = load_instruction_from_packages(instruction_byte, package_instruction_loaders);
    
    return match next_instruction {
        Some(instruction) => {
            instruction
        },
        None => panic!("Unknown instruction: {}", as_hex!(instruction_byte)),
    }
}

fn load_prefix_instruction(instruction_byte: u8) -> Box<dyn Instruction> {
    let package_instruction_loaders: Vec<PackageInstructionLoader> = vec![
        bit::instructions::load_instruction,
    ];
    
    let next_instruction = load_instruction_from_packages(instruction_byte, package_instruction_loaders);
    
    return match next_instruction {
        Some(instruction) => {
            instruction
        },
        None => panic!("Unknown prefix instruction: {}", as_hex!(instruction_byte)),
    }
}

fn load_instruction_from_packages(instruction_byte: u8, package_instruction_loaders: Vec<PackageInstructionLoader>) -> Option<Box<dyn Instruction>> {
    let next_instruction = package_instruction_loaders.iter().find_map(
        |load_from_package| load_from_package(instruction_byte)
    );
    
    return next_instruction;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::testing::build_test_program_counter;
    
    #[test]
    fn test_load_instruction_returns_instruction() {
        const ADD_INSTRUCTION: u8 = 0x87;
        let mut program = build_test_program_counter();
        program._memory.borrow_mut().write_byte(0x0000, ADD_INSTRUCTION);
        
        load_instruction(&mut program);
        
        // Not sure if there'e anything I can assert on
    }
    
    // TODO: Uncomment once Prefix has its first actual instruction
    /*#[test]
    fn test_load_instruction_prefix_instruction_returns_instruction() {
        const BIT_INSTRUCTION: u8 = 0x40;
        let mut program = build_test_program_counter();
        program._memory.borrow_mut().write_byte(0x0000, PREFIX_INSTRUCTION);
        program._memory.borrow_mut().write_byte(0x0001, BIT_INSTRUCTION);
        
        load_instruction(&mut program);
        
        // Not sure if there'e anything I can assert on
    }*/
}
