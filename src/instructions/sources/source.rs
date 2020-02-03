use crate::registers::registers::Registers;
use mockall::{automock};

#[automock]
pub trait Source {
    fn read(&self, registers: &Registers) -> u8;
}
