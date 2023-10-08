extern crate derive_more;
use super::base::{Height, Pixels, Width};
use super::text::Text;
use super::tiles::{Tiles1x, Tiles2x};
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PosX<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PosY<T>(pub T);

impl Add<Width<Pixels>> for PosX<Pixels> {
    type Output = Self;

    fn add(self, rhs: Width<Pixels>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Height<Pixels>> for PosY<Pixels> {
    type Output = Self;

    fn add(self, rhs: Height<Pixels>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Width<Tiles1x>> for PosX<Tiles1x> {
    type Output = Self;

    fn add(self, rhs: Width<Tiles1x>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Height<Tiles1x>> for PosY<Tiles1x> {
    type Output = Self;

    fn add(self, rhs: Height<Tiles1x>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Width<Tiles2x>> for PosX<Tiles2x> {
    type Output = Self;

    fn add(self, rhs: Width<Tiles2x>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Height<Tiles2x>> for PosY<Tiles2x> {
    type Output = Self;

    fn add(self, rhs: Height<Tiles2x>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Width<Text>> for PosX<Text> {
    type Output = Self;

    fn add(self, rhs: Width<Text>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Height<Text>> for PosY<Text> {
    type Output = Self;

    fn add(self, rhs: Height<Text>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
