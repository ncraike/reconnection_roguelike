extern crate derive_more;
use super::traits::Unit;
use derive_more::{Add, Mul, Sub};
use std::ops::Add as AddTrait;
use std::ops::Sub as SubTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Mul)]
pub struct Width<T: Unit>(pub T);

impl<T: Unit> Width<T> {
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Mul)]
pub struct Height<T: Unit>(pub T);

impl<T: Unit> Height<T> {
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PosX<T: Unit>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PosY<T: Unit>(pub T);

impl<T: Unit + AddTrait<Output = T>> AddTrait<Width<T>> for PosX<T> {
    type Output = Self;

    fn add(self, rhs: Width<T>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Unit + SubTrait<Output = T>> SubTrait<PosX<T>> for PosX<T> {
    type Output = Width<T>;

    fn sub(self, rhs: PosX<T>) -> Self::Output {
        Width::<T>(self.0 - rhs.0)
    }
}

impl<T: Unit + AddTrait<Output = T>> AddTrait<Height<T>> for PosY<T> {
    type Output = Self;

    fn add(self, rhs: Height<T>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Unit + SubTrait<Output = T>> SubTrait<PosY<T>> for PosY<T> {
    type Output = Height<T>;

    fn sub(self, rhs: PosY<T>) -> Self::Output {
        Height::<T>(self.0 - rhs.0)
    }
}
