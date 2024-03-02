use std::ops::{Add as AddTrait, Div as DivTrait, Mul as MulTrait, Sub as SubTrait};
extern crate derive_more;
use super::Unit;
use derive_more::{Add, Sub};

mod width;

pub use width::Width;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub)]
pub struct Height<T: Unit>(pub T);

impl<T: Unit> Height<T> {
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    pub fn to_primitive(&self) -> i32 {
        self.0.to_primitive()
    }
}

impl<T: Unit + MulTrait<i32, Output = T>> MulTrait<i32> for Height<T> {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl<T: Unit + DivTrait<i32, Output = T>> DivTrait<i32> for Height<T> {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size2D<T: Unit> {
    pub width: Width<T>,
    pub height: Height<T>,
}

impl<T: Unit + AddTrait<T> + SubTrait<T> + MulTrait<i32> + DivTrait<i32>> Size2D<T> {
    pub fn nothing() -> Self {
        Self {
            width: Width(Unit::zero()),
            height: Height(Unit::zero()),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            width: self.width.abs(),
            height: self.height.abs(),
        }
    }
}

impl<T: Unit + MulTrait<i32, Output = T>> MulTrait<i32> for Size2D<T> {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl<T: Unit + DivTrait<i32, Output = T>> DivTrait<i32> for Size2D<T> {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}
