use super::super::super::registers::registers::Registers;

pub trait Destination {
    fn assign(&self, registers: &mut Registers, new_value: u8);
}
