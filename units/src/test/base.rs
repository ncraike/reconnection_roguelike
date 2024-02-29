extern crate derive_more;
use crate::Unit;
use derive_more::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Mul, Div)]
pub struct MyUnitI32(pub i32);

impl Unit for MyUnitI32 {
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

#[test]
fn unit_i32_new() {
    assert_eq!(MyUnitI32::new(3), MyUnitI32(3));
}

#[test]
fn unit_i32_zero() {
    assert_eq!(MyUnitI32::zero(), MyUnitI32(0));
    assert_eq!(MyUnitI32::zero().to_primitive(), 0);
}

#[test]
fn unit_i32_to_primitive() {
    assert_eq!(MyUnitI32(7).to_primitive(), 7);
}

#[test]
fn unit_i32_mul() {
    assert_eq!(MyUnitI32(3) * 4, MyUnitI32(12));
}

#[test]
fn unit_i32_div() {
    assert_eq!(MyUnitI32(6) / 2, MyUnitI32(3));
}
