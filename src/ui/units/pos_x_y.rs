extern crate derive_more;
use super::base::{Height, Width};
use super::traits::Unit;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PosX<T: Unit>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PosY<T: Unit>(pub T);

impl<T: Unit + Add<Output = T>> Add<Width<T>> for PosX<T> {
    type Output = Self;

    fn add(self, rhs: Width<T>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Unit + Sub<Output = T>> Sub<PosX<T>> for PosX<T> {
    type Output = Width<T>;

    fn sub(self, rhs: PosX<T>) -> Self::Output {
        Width::<T>(self.0 - rhs.0)
    }
}

impl<T: Unit + Add<Output = T>> Add<Height<T>> for PosY<T> {
    type Output = Self;

    fn add(self, rhs: Height<T>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Unit + Sub<Output = T>> Sub<PosY<T>> for PosY<T> {
    type Output = Height<T>;

    fn sub(self, rhs: PosY<T>) -> Self::Output {
        Height::<T>(self.0 - rhs.0)
    }
}
