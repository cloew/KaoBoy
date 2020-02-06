use super::super::super::instruction_context::InstructionContext;

pub trait Destination {
    fn assign(&self, context: &mut InstructionContext, new_value: u8);
}
