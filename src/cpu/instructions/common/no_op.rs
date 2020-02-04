use super::super::super::registers::registers::Registers;

pub fn no_op(registers: &mut Registers, value: u8) -> u8 {
    return value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{as_hex};
    
    #[test]
    fn test_run_calls_source_op_and_destination() {
        const VALUE: u8 = 0x12;
        let mut registers = Registers::new();
        
        let result = no_op(&mut registers, VALUE);
        
        assert_eq!(as_hex!(result), as_hex!(VALUE));
    }
}
