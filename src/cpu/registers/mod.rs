pub mod registers;
pub mod register_names;
pub mod double_register_names;

mod double_register;
mod register;
mod register_flag;

pub use registers::Registers;
pub use register_names::RegisterName;
pub use double_register_names::DoubleRegisterName;
