use super::super::super::InstructionContext;

pub trait ShortDestination {
    fn assign(&self, context: &mut InstructionContext, new_value: u16);
}
