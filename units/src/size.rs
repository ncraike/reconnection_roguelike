extern crate derive_more;
use super::UnitI32;
use derive_more::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Mul, Div)]
pub struct WidthI32<T: UnitI32>(pub T);

impl<T: UnitI32> WidthI32<T> {
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    pub fn to_primitive(&self) -> i32 {
        self.0.to_primitive()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Mul, Div)]
pub struct HeightI32<T: UnitI32>(pub T);

impl<T: UnitI32> HeightI32<T> {
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    pub fn to_primitive(&self) -> i32 {
        self.0.to_primitive()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size2DI32<T: UnitI32> {
    pub width: WidthI32<T>,
    pub height: HeightI32<T>,
}

impl<T: UnitI32> Size2DI32<T> {
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
