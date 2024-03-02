use std::ops::{Add as AddTrait, Div as DivTrait, Mul as MulTrait, Sub as SubTrait};
extern crate derive_more;
use super::Unit;

mod height;
mod width;

pub use height::Height;
pub use width::Width;

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
