use crate::registers::registers::Registers;
use mockall::{automock};

#[automock]
pub trait Destination {
    fn assign(&self, registers: &mut Registers, new_value: u8);
}
