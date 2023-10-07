use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixels(pub i32);

impl Add<Pixels> for Pixels {
    type Output = Self;

    fn add(self, rhs: Pixels) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Mul<i32> for Pixels {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Width<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Height<T>(pub T);
