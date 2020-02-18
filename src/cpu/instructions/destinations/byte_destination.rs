use super::super::super::InstructionContext;

pub trait ByteDestination {
    fn assign(&self, context: &mut InstructionContext, new_value: u8);
}
