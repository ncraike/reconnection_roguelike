use super::Box2D;
use crate::{Height, Position2D, Size2D, Unit, Width};
use std::ops::{Add, Sub};

impl<T: Unit + Copy + Add<Output = T> + Sub<Output = T> + Ord> Box2D<T> {
    pub fn new_from_position_and_size(position: Position2D<T>, size: Size2D<T>) -> Self {
        Self {
            p1: position,
            p2: position + size,
        }
    }

    pub fn new_from_size(size: Size2D<T>) -> Self {
        Self::new_from_position_and_size(Position2D::origin(), size)
    }

    pub fn new_from_width_height(width: Width<T>, height: Height<T>) -> Self {
        Self::new_from_size(Size2D {
            width: width,
            height: height,
        })
    }
}
