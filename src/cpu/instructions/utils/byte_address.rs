use crate::build_u16;

pub fn build_full_address(byte_address: u8) -> u16 {
    return build_u16!(0xFF, byte_address);
}
