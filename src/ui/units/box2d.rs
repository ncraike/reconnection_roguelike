use super::base::{Height, PosX, PosY, Width};
use super::point_and_size::{Point2D, Size2D};
use super::traits::Unit;
use std::cmp::{max, min};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Box2D<T: Unit> {
    pub p1: Point2D<T>,
    pub p2: Point2D<T>,
}

impl<T: Unit + Copy + Add<Output = T> + Sub<Output = T> + Ord> Box2D<T> {
    pub fn new_from_p1_p2(p1: Point2D<T>, p2: Point2D<T>) -> Self {
        Self { p1: p1, p2: p2 }
    }

    pub fn new_from_point_and_size(point: Point2D<T>, size: Size2D<T>) -> Self {
        Self::new_from_p1_p2(point, point + size)
    }

    pub fn new_from_size(size: Size2D<T>) -> Self {
        Self::new_from_point_and_size(Point2D::<T>::origin(), size)
    }

    pub fn new_from_width_height(width: Width<T>, height: Height<T>) -> Self {
        Self::new_from_size(Size2D::new_from_width_height(width, height))
    }

    pub fn x1(&self) -> PosX<T> {
        self.p1.x
    }

    pub fn y1(&self) -> PosY<T> {
        self.p1.y
    }

    pub fn x2(&self) -> PosX<T> {
        self.p2.x
    }

    pub fn y2(&self) -> PosY<T> {
        self.p2.y
    }

    pub fn size(&self) -> Size2D<T> {
        self.p2 - self.p1
    }

    pub fn width(&self) -> Width<T> {
        self.size().width
    }

    pub fn height(&self) -> Height<T> {
        self.size().height
    }

    pub fn normalize(&self) -> Self {
        Self::new_from_p1_p2(
            Point2D::<T> {
                x: min(self.p1.x, self.p2.x),
                y: min(self.p1.y, self.p2.y),
            },
            Point2D::<T> {
                x: max(self.p1.x, self.p2.x),
                y: max(self.p1.y, self.p2.y),
            },
        )
    }
}
