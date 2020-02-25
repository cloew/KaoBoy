pub mod byte_destination;
pub mod double_register_destination;
pub mod register_destination;
pub mod short_destination;
pub mod stack_pointer_destination;

pub use byte_destination::ByteDestination;
pub use double_register_destination::DoubleRegisterDestination;
pub use register_destination::RegisterDestination;
pub use short_destination::ShortDestination;
pub use stack_pointer_destination::StackPointerDestination;
