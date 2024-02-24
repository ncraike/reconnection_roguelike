extern crate derive_more;
use super::base::UnitI32;
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


pub struct Size2DI32<T: UnitI32> {
    pub width: WidthI32<T>,
    pub height: HeightI32<T>,
}

impl<T: UnitI32> Size2DI32<T> {
    pub fn new_from_width_height(w: WidthI32<T>, h: HeightI32<T>) -> Self {
        Self {
            width: w,
            height: h,
        }
    }

    pub fn nothing() -> Self {
        Self::new_from_width_height(WidthI32(UnitI32::zero()), HeightI32(UnitI32::zero()))
    }

    pub fn abs(&self) -> Self {
        Self {
            width: self.width.abs(),
            height: self.height.abs(),
        }
    }
}
