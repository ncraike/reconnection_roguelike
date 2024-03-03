extern crate derive_more;
use crate::Unit;
use derive_more::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Mul, Div)]
pub struct MyUnit(pub i32);

impl Unit for MyUnit {
    fn new(value: i32) -> Self {
        Self(value)
    }

    fn zero() -> Self {
        Self(0 as i32)
    }

    fn to_primitive(&self) -> i32 {
        self.0
    }

    fn abs(&self) -> Self {
        Self(self.0.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::{MyUnit, Unit};

    #[test]
    fn new() {
        assert_eq!(MyUnit::new(3), MyUnit(3));
    }

    #[test]
    fn zero() {
        assert_eq!(MyUnit::zero(), MyUnit(0));
        assert_eq!(MyUnit::zero().to_primitive(), 0);
    }

    #[test]
    fn to_primitive() {
        assert_eq!(MyUnit(7).to_primitive(), 7);
    }

    #[test]
    fn mul() {
        assert_eq!(MyUnit(3) * 4, MyUnit(12));
    }

    #[test]
    fn div() {
        assert_eq!(MyUnit(6) / 2, MyUnit(3));
    }
}
