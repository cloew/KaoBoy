#[macro_export]
macro_rules! build_u16 {
    ($upper:expr, $lower:expr) => (($upper as u16) << 8 | ($lower as u16));
}

#[macro_export]
macro_rules! get_lower_u8 {
    ($value:expr) => ($value as u8);
}

#[macro_export]
macro_rules! get_upper_u8 {
    ($value:expr) => ((($value & 0xFF00) >> 8) as u8);
}

