pub mod addressed_by_byte_destination;
pub mod addressed_by_short_destination;
pub mod byte_destination;
pub mod double_register_destination;
pub mod register_destination;
pub mod short_destination;
pub mod stack_pointer_destination;

pub use addressed_by_byte_destination::AddressedByByteDestination;
pub use addressed_by_short_destination::AddressedByShortDestination;
pub use byte_destination::ByteDestination;
pub use double_register_destination::DoubleRegisterDestination;
pub use register_destination::RegisterDestination;
pub use short_destination::ShortDestination;
pub use stack_pointer_destination::StackPointerDestination;
