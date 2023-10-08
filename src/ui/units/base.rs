extern crate derive_more;
use super::traits::Unit;
use derive_more::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Add, Mul)]
pub struct Pixels(pub i32);

impl Unit for Pixels {
    type ValueType = i32;

    fn new(value: i32) -> Pixels {
        Pixels(value)
    }

    fn zero() -> Pixels {
        Pixels(0)
    }

    fn value(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Add, Mul)]
pub struct Width<T: Unit>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Add, Mul)]
pub struct Height<T: Unit>(pub T);
