use std::ops::{Add as AddTrait, Div as DivTrait, Mul as MulTrait, Sub as SubTrait};
extern crate derive_more;
use super::UnitI32;
use derive_more::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub)]
pub struct WidthI32<T: UnitI32>(pub T);

impl<T: UnitI32> WidthI32<T> {
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    pub fn to_primitive(&self) -> i32 {
        self.0.to_primitive()
    }
}

impl<T: UnitI32 + MulTrait<i32, Output = T>> MulTrait<i32> for WidthI32<T> {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl<T: UnitI32 + DivTrait<i32, Output = T>> DivTrait<i32> for WidthI32<T> {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub)]
pub struct HeightI32<T: UnitI32>(pub T);

impl<T: UnitI32> HeightI32<T> {
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    pub fn to_primitive(&self) -> i32 {
        self.0.to_primitive()
    }
}

impl<T: UnitI32 + MulTrait<i32, Output = T>> MulTrait<i32> for HeightI32<T> {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl<T: UnitI32 + DivTrait<i32, Output = T>> DivTrait<i32> for HeightI32<T> {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size2DI32<T: UnitI32> {
    pub width: WidthI32<T>,
    pub height: HeightI32<T>,
}

impl<T: UnitI32 + AddTrait<T> + SubTrait<T> + MulTrait<i32> + DivTrait<i32>> Size2DI32<T> {
    pub fn nothing() -> Self {
        Self {
            width: WidthI32(UnitI32::zero()),
            height: HeightI32(UnitI32::zero()),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            width: self.width.abs(),
            height: self.height.abs(),
        }
    }
}

impl<T: UnitI32 + MulTrait<i32, Output = T>> MulTrait<i32> for Size2DI32<T> {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl<T: UnitI32 + DivTrait<i32, Output = T>> DivTrait<i32> for Size2DI32<T> {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}
