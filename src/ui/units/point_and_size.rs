extern crate derive_more;
use super::base::{Height, PosX, PosY, Width};
use super::box2d::Box2D;
use super::traits::Unit;
use derive_more::{Add, Mul};
use std::ops::Add as AddTrait;
use std::ops::Sub as SubTrait;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D<T: Unit> {
    pub x: PosX<T>,
    pub y: PosY<T>,
}

impl<T: Unit + Copy + AddTrait<Output = T> + SubTrait<Output = T> + Ord> Point2D<T> {
    pub fn new_from_x_y(x: PosX<T>, y: PosY<T>) -> Self {
        Self { x: x, y: y }
    }

    pub fn origin() -> Self {
        Self::new_from_x_y(PosX(Unit::zero()), PosY(Unit::zero()))
    }

    pub fn new_box2d_from_other_point(&self, p2: Point2D<T>) -> Box2D<T> {
        Box2D::new_from_p1_p2(*self, p2)
    }

    pub fn new_box2d_from_size(&self, size: Size2D<T>) -> Box2D<T> {
        Box2D::new_from_point_and_size(*self, size)
    }

    pub fn new_box2d_from_width_height(&self, width: Width<T>, height: Height<T>) -> Box2D<T> {
        Box2D::new_from_point_and_size(*self, Size2D::<T>::new_from_width_height(width, height))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Add, Mul)]
pub struct Size2D<T: Unit> {
    pub w: Width<T>,
    pub h: Height<T>,
}

impl<T: Unit> Size2D<T> {
    pub fn new_from_width_height(w: Width<T>, h: Height<T>) -> Self {
        Self { w: w, h: h }
    }

    pub fn nothing() -> Self {
        Self::new_from_width_height(Width(Unit::zero()), Height(Unit::zero()))
    }

    pub fn abs(&self) -> Self {
        Self {
            w: self.w.abs(),
            h: self.h.abs(),
        }
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
