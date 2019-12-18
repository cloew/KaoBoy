#[macro_export]
macro_rules! as_hex {
    ($value:expr) => (format!("0x{:X}", $value));
}
