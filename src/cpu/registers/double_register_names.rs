use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum DoubleRegisterName {
    AF = 0,
    BC = 1,
    DE = 2,
    HL = 3,
}

impl fmt::Display for DoubleRegisterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
