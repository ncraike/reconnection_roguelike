extern crate derive_more;
use super::base::{Height, Width};
use super::pos_x_y::{PosX, PosY};
use super::traits::Unit;
use derive_more::{Add, Mul};
use std::ops::Add as AddTrait;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D<T: Unit> {
    pub x: PosX<T>,
    pub y: PosY<T>,
}

impl<T: Unit> Point2D<T> {
    pub fn origin() -> Point2D<T> {
        return Point2D::<T> {
            x: PosX::<T>(Unit::zero()),
            y: PosY::<T>(Unit::zero()),
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Add, Mul)]
pub struct Size2D<T: Unit> {
    pub w: Width<T>,
    pub h: Height<T>,
}

impl<T: Unit + std::ops::Add<Output = T>> AddTrait<Size2D<T>> for Point2D<T> {
    type Output = Self;

    fn add(self, rhs: Size2D<T>) -> Self::Output {
        Self {
            x: self.x + rhs.w,
            y: self.y + rhs.h,
        }
    }
}
