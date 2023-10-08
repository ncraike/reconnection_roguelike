extern crate derive_more;
use super::base::{Height, Pixels, Width};
use super::pos_x_y::{PosX, PosY};
use super::text::Text;
use super::tiles::{Tiles1x, Tiles2x};
use derive_more::{Add, Mul};
use std::ops::Add as AddTrait;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D<T> {
    pub x: PosX<T>,
    pub y: PosY<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Add, Mul)]
pub struct Size2D<T> {
    pub w: Width<T>,
    pub h: Height<T>,
}

impl AddTrait<Size2D<Pixels>> for Point2D<Pixels> {
    type Output = Self;

    fn add(self, rhs: Size2D<Pixels>) -> Self::Output {
        Self {
            x: self.x + rhs.w,
            y: self.y + rhs.h,
        }
    }
}

impl AddTrait<Size2D<Tiles1x>> for Point2D<Tiles1x> {
    type Output = Self;

    fn add(self, rhs: Size2D<Tiles1x>) -> Self::Output {
        Self {
            x: self.x + rhs.w,
            y: self.y + rhs.h,
        }
    }
}

impl AddTrait<Size2D<Tiles2x>> for Point2D<Tiles2x> {
    type Output = Self;

    fn add(self, rhs: Size2D<Tiles2x>) -> Self::Output {
        Self {
            x: self.x + rhs.w,
            y: self.y + rhs.h,
        }
    }
}

impl AddTrait<Size2D<Text>> for Point2D<Text> {
    type Output = Self;

    fn add(self, rhs: Size2D<Text>) -> Self::Output {
        Self {
            x: self.x + rhs.w,
            y: self.y + rhs.h,
        }
    }
}
