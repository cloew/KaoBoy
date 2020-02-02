
pub fn check_half_carry(left_value: u8, right_value: u8) -> bool {
    return get_lower_nibble(left_value) + get_lower_nibble(right_value) > 0xF;
}

fn get_lower_nibble(value: u8) -> u8 {
    return value & 0xF;
}
