
pub fn check_half_carry(left_value: u8, right_value: u8) -> bool {
    return get_lower_nibble(left_value) + get_lower_nibble(right_value) > 0xF;
}

pub fn check_half_borrow(left_value: u8, right_value: u8) -> bool {
    return get_lower_nibble(right_value) > get_lower_nibble(left_value);
}

fn get_lower_nibble(value: u8) -> u8 {
    return value & 0xF;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_check_half_carry_no_lower_nibble_returns_false() {
        const LEFT: u8 = 0xB0;
        const RIGHT: u8 = 0x10;
        
        let result = check_half_carry(LEFT, RIGHT);
        
        assert_eq!(result, false);
    }
    
    #[test]
    fn test_check_half_carry_lower_nibble_overflowed_returns_true() {
        const LEFT: u8 = 0xFF;
        const RIGHT: u8 = 0x1;
        
        let result = check_half_carry(LEFT, RIGHT);
        
        assert_eq!(result, true);
    }
    
    #[test]
    fn test_check_half_borrow_no_lower_nibble_overflow_returns_false() {
        const LEFT: u8 = 0x20;
        const RIGHT: u8 = 0x10;
        
        let result = check_half_borrow(LEFT, RIGHT);
        
        assert_eq!(result, false);
    }
    
    #[test]
    fn test_check_half_borrow_lower_nibble_overflowed_returns_true() {
        const LEFT: u8 = 0x10;
        const RIGHT: u8 = 0x1;
        
        let result = check_half_borrow(LEFT, RIGHT);
        
        assert_eq!(result, true);
    }
}