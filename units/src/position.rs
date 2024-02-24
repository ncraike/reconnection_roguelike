extern crate derive_more;
use super::base::UnitI32;
use super::size::{HeightI32, WidthI32};
use std::ops::Add as AddTrait;
use std::ops::Sub as SubTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PosXI32<T: UnitI32>(pub T);

impl<T: UnitI32> PosXI32<T> {
    pub fn to_primitive(&self) -> i32 {
        self.0.to_primitive()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PosYI32<T: UnitI32>(pub T);

impl<T: UnitI32> PosYI32<T> {
    pub fn to_primitive(&self) -> i32 {
        self.0.to_primitive()
    }
}

impl<T: UnitI32 + AddTrait<Output = T>> AddTrait<WidthI32<T>> for PosXI32<T> {
    type Output = Self;

    fn add(self, rhs: WidthI32<T>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: UnitI32 + SubTrait<Output = T>> SubTrait<PosXI32<T>> for PosXI32<T> {
    type Output = WidthI32<T>;

    fn sub(self, rhs: PosXI32<T>) -> Self::Output {
        WidthI32::<T>(self.0 - rhs.0)
    }
}

impl<T: UnitI32 + SubTrait<Output = T>> SubTrait<WidthI32<T>> for PosXI32<T> {
    type Output = PosXI32<T>;

    fn sub(self, rhs: WidthI32<T>) -> Self::Output {
        PosXI32::<T>(self.0 - rhs.0)
    }
}

impl<T: UnitI32 + AddTrait<Output = T>> AddTrait<HeightI32<T>> for PosYI32<T> {
    type Output = Self;

    fn add(self, rhs: HeightI32<T>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: UnitI32 + SubTrait<Output = T>> SubTrait<PosYI32<T>> for PosYI32<T> {
    type Output = HeightI32<T>;

    fn sub(self, rhs: PosYI32<T>) -> Self::Output {
        HeightI32::<T>(self.0 - rhs.0)
    }
}
impl<T: UnitI32 + SubTrait<Output = T>> SubTrait<HeightI32<T>> for PosYI32<T> {
    type Output = PosYI32<T>;

    fn sub(self, rhs: HeightI32<T>) -> Self::Output {
        PosYI32::<T>(self.0 - rhs.0)
    }
}
