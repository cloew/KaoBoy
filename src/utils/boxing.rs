#[macro_export]
macro_rules! boxed {
    ($value:expr) => (Box::new($value));
}

#[macro_export]
macro_rules! optional_boxed {
    ($value:expr) => (Some(boxed!($value)));
}

