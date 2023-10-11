extern crate derive_more;
use super::base::{Height, PosX, PosY, Width};
use super::traits::Unit;
use derive_more::{Add, Mul};
use std::ops::Add as AddTrait;
use std::ops::Sub as SubTrait;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D<T: Unit> {
    pub x: PosX<T>,
    pub y: PosY<T>,
}

impl<T: Unit> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x: PosX(x),
            y: PosY(y),
        }
    }

    pub fn origin() -> Self {
        Self::new(Unit::zero(), Unit::zero())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Add, Mul)]
pub struct Size2D<T: Unit> {
    pub w: Width<T>,
    pub h: Height<T>,
}

impl<T: Unit> Size2D<T> {
    pub fn new(w: T, h: T) -> Self {
        Self {
            w: Width(w),
            h: Height(h),
        }
    }

    pub fn nothing() -> Self {
        Self::new(Unit::zero(), Unit::zero())
    }
}

impl<T: Unit + AddTrait<Output = T>> AddTrait<Size2D<T>> for Point2D<T> {
    type Output = Self;

    fn add(self, rhs: Size2D<T>) -> Self::Output {
        Self {
            x: self.x + rhs.w,
            y: self.y + rhs.h,
        }
    }
}

impl<T: Unit + SubTrait<Output = T>> SubTrait<Point2D<T>> for Point2D<T> {
    type Output = Size2D<T>;

    fn sub(self, rhs: Point2D<T>) -> Size2D<T> {
        Size2D::<T> {
            w: self.x - rhs.x,
            h: self.y - rhs.y,
        }
    }
}
