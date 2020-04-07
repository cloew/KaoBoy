use super::super::instruction::Instruction;
use super::super::common::{UnaryByteOp, UnaryShortOp};
use super::super::sources::{AddressedByByteSource, AddressedByShortSource, ConstantByteSource, ConstantShortSource, RegisterSource};
use super::super::destinations::{AddressedByByteDestination, AddressedByDoubleRegisterDestination, DoubleRegisterDestination, RegisterDestination, StackPointerDestination};
use super::super::super::registers::{DoubleRegisterName, RegisterName};
use crate::{boxed, optional_boxed};

fn build_load_instruction(source_name: RegisterName, destination_name: RegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryByteOp::new_no_op(
            boxed!(RegisterSource::new(source_name)),
            boxed!(RegisterDestination::new(destination_name))
        )
    );
}

fn build_load_instruction_from_constant_byte(destination_name: RegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryByteOp::new_no_op(
            boxed!(ConstantByteSource::new()),
            boxed!(RegisterDestination::new(destination_name))
        )
    );
}

fn build_load_into_address_instruction(source_name: RegisterName, destination_name: DoubleRegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryByteOp::new_no_op(
            boxed!(RegisterSource::new(source_name)),
            boxed!(AddressedByDoubleRegisterDestination::new(destination_name))
        )
    );
}

fn build_load_into_address_instruction_from_constant_byte(destination_name: DoubleRegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryByteOp::new_no_op(
            boxed!(ConstantByteSource::new()),
            boxed!(AddressedByDoubleRegisterDestination::new(destination_name))
        )
    );
}

fn build_load_from_address_instruction(source_name: DoubleRegisterName, destination_name: RegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryByteOp::new_no_op(
            boxed!(AddressedByShortSource::new_from_register(source_name)),
            boxed!(RegisterDestination::new(destination_name))
        )
    );
}

fn build_load_from_address_instruction_from_constant_short(destination_name: RegisterName) -> Option<Box<dyn Instruction>> {
    return optional_boxed!(
        UnaryByteOp::new_no_op(
            boxed!(AddressedByShortSource::new_from_constant()),
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
        0x06 => build_load_instruction_from_constant_byte(RegisterName::B),
        // Load into Register C
        0x48 => build_load_instruction(RegisterName::B, RegisterName::C),
        0x49 => build_load_instruction(RegisterName::C, RegisterName::C),
        0x4A => build_load_instruction(RegisterName::D, RegisterName::C),
        0x4B => build_load_instruction(RegisterName::E, RegisterName::C),
        0x4C => build_load_instruction(RegisterName::H, RegisterName::C),
        0x4D => build_load_instruction(RegisterName::L, RegisterName::C),
        0x4F => build_load_instruction(RegisterName::A, RegisterName::C),
        0x0E => build_load_instruction_from_constant_byte(RegisterName::C),
        // Load into Register D
        0x50 => build_load_instruction(RegisterName::B, RegisterName::D),
        0x51 => build_load_instruction(RegisterName::C, RegisterName::D),
        0x52 => build_load_instruction(RegisterName::D, RegisterName::D),
        0x53 => build_load_instruction(RegisterName::E, RegisterName::D),
        0x54 => build_load_instruction(RegisterName::H, RegisterName::D),
        0x55 => build_load_instruction(RegisterName::L, RegisterName::D),
        0x57 => build_load_instruction(RegisterName::A, RegisterName::D),
        0x16 => build_load_instruction_from_constant_byte(RegisterName::D),
        // Load into Register E
        0x58 => build_load_instruction(RegisterName::B, RegisterName::E),
        0x59 => build_load_instruction(RegisterName::C, RegisterName::E),
        0x5A => build_load_instruction(RegisterName::D, RegisterName::E),
        0x5B => build_load_instruction(RegisterName::E, RegisterName::E),
        0x5C => build_load_instruction(RegisterName::H, RegisterName::E),
        0x5D => build_load_instruction(RegisterName::L, RegisterName::E),
        0x5F => build_load_instruction(RegisterName::A, RegisterName::E),
        0x1E => build_load_instruction_from_constant_byte(RegisterName::E),
        // Load into Register H
        0x60 => build_load_instruction(RegisterName::B, RegisterName::H),
        0x61 => build_load_instruction(RegisterName::C, RegisterName::H),
        0x62 => build_load_instruction(RegisterName::D, RegisterName::H),
        0x63 => build_load_instruction(RegisterName::E, RegisterName::H),
        0x64 => build_load_instruction(RegisterName::H, RegisterName::H),
        0x65 => build_load_instruction(RegisterName::L, RegisterName::H),
        0x67 => build_load_instruction(RegisterName::A, RegisterName::H),
        0x26 => build_load_instruction_from_constant_byte(RegisterName::H),
        // Load into Register L
        0x68 => build_load_instruction(RegisterName::B, RegisterName::L),
        0x69 => build_load_instruction(RegisterName::C, RegisterName::L),
        0x6A => build_load_instruction(RegisterName::D, RegisterName::L),
        0x6B => build_load_instruction(RegisterName::E, RegisterName::L),
        0x6C => build_load_instruction(RegisterName::H, RegisterName::L),
        0x6D => build_load_instruction(RegisterName::L, RegisterName::L),
        0x6F => build_load_instruction(RegisterName::A, RegisterName::L),
        0x2E => build_load_instruction_from_constant_byte(RegisterName::L),
        // Load into Register A
        0x78 => build_load_instruction(RegisterName::B, RegisterName::A),
        0x79 => build_load_instruction(RegisterName::C, RegisterName::A),
        0x7A => build_load_instruction(RegisterName::D, RegisterName::A),
        0x7B => build_load_instruction(RegisterName::E, RegisterName::A),
        0x7C => build_load_instruction(RegisterName::H, RegisterName::A),
        0x7D => build_load_instruction(RegisterName::L, RegisterName::A),
        0x7F => build_load_instruction(RegisterName::A, RegisterName::A),
        0x3E => build_load_instruction_from_constant_byte(RegisterName::A),
        // Load into Addressed by HL
        0x70 => build_load_into_address_instruction(RegisterName::B, DoubleRegisterName::HL),
        0x71 => build_load_into_address_instruction(RegisterName::C, DoubleRegisterName::HL),
        0x72 => build_load_into_address_instruction(RegisterName::D, DoubleRegisterName::HL),
        0x73 => build_load_into_address_instruction(RegisterName::E, DoubleRegisterName::HL),
        0x74 => build_load_into_address_instruction(RegisterName::H, DoubleRegisterName::HL),
        0x75 => build_load_into_address_instruction(RegisterName::L, DoubleRegisterName::HL),
        0x77 => build_load_into_address_instruction(RegisterName::A, DoubleRegisterName::HL),
        0x36 => build_load_into_address_instruction_from_constant_byte(DoubleRegisterName::HL),
        // Load Short Fields
        0x01 => optional_boxed!(UnaryShortOp::new_no_op(boxed!(ConstantShortSource::new()), boxed!(DoubleRegisterDestination::new(DoubleRegisterName::BC)))),
        0x11 => optional_boxed!(UnaryShortOp::new_no_op(boxed!(ConstantShortSource::new()), boxed!(DoubleRegisterDestination::new(DoubleRegisterName::DE)))),
        0x21 => optional_boxed!(UnaryShortOp::new_no_op(boxed!(ConstantShortSource::new()), boxed!(DoubleRegisterDestination::new(DoubleRegisterName::HL)))),
        0x31 => optional_boxed!(UnaryShortOp::new_no_op(boxed!(ConstantShortSource::new()), boxed!(StackPointerDestination::new()))),
        // Load Addressed by Double Register Fields
        0x02 => build_load_into_address_instruction(RegisterName::A, DoubleRegisterName::BC),
        0x12 => build_load_into_address_instruction(RegisterName::A, DoubleRegisterName::DE),
        // Load Addressed by Double Register Fields, then modify the register
        0x22 => optional_boxed!(
                    UnaryByteOp::new_no_op(
                        boxed!((RegisterSource::new(RegisterName::A))),
                        boxed!(AddressedByDoubleRegisterDestination::new_assign_then_increment(DoubleRegisterName::HL))
                    )
                ),
        0x32 => optional_boxed!(
                    UnaryByteOp::new_no_op(
                        boxed!((RegisterSource::new(RegisterName::A))),
                        boxed!(AddressedByDoubleRegisterDestination::new_assign_then_decrement(DoubleRegisterName::HL))
                    )
                ),
        // Load Addressed by Byte Fields
        0xE0 => optional_boxed!(
                    UnaryByteOp::new_no_op(
                        boxed!((RegisterSource::new(RegisterName::A))),
                        boxed!(AddressedByByteDestination::new(boxed!(ConstantByteSource::new())))
                    )
                ),
        0xE2 => optional_boxed!(
                    UnaryByteOp::new_no_op(
                        boxed!((RegisterSource::new(RegisterName::A))),
                        boxed!(AddressedByByteDestination::new(boxed!(RegisterSource::new(RegisterName::C))))
                    )
                ),
        0xF0 => optional_boxed!(
                    UnaryByteOp::new_no_op(
                        boxed!(AddressedByByteSource::new(boxed!(ConstantByteSource::new()))),
                        boxed!((RegisterDestination::new(RegisterName::A)))
                    )
                ),
        0xF2 => optional_boxed!(
                    UnaryByteOp::new_no_op(
                        boxed!(AddressedByByteSource::new(boxed!(RegisterSource::new(RegisterName::C)))),
                        boxed!((RegisterDestination::new(RegisterName::A)))
                    )
                ),
        // Load from Addressed by Short Fields
        0x0A => build_load_from_address_instruction(DoubleRegisterName::BC, RegisterName::A),
        0x1A => build_load_from_address_instruction(DoubleRegisterName::DE, RegisterName::A),
        0xFA => build_load_from_address_instruction_from_constant_short(RegisterName::A),
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
