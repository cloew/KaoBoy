use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum RegisterName {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    H = 6,
    L = 7,
}

impl fmt::Display for RegisterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
