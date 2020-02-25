pub mod registers;
pub mod register_names;
pub mod double_register_names;

mod double_register;
mod register;
mod register_flag;

pub use self::registers::Registers;
pub use self::register_names::RegisterName;
pub use self::double_register_names::DoubleRegisterName;
