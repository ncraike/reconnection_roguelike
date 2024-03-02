extern crate derive_more;
use super::{Height, Size2D, Unit, Width};
use bracket_geometry::prelude::Point;
use std::ops::{Add as AddTrait, Sub as SubTrait};

mod pos_x;
pub use pos_x::PosX;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PosY<T: Unit>(pub T);

impl<T: Unit> PosY<T> {
    pub fn to_primitive(&self) -> i32 {
        self.0.to_primitive()
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

impl<T: Unit + SubTrait<Output = T>> SubTrait<Height<T>> for PosY<T> {
    type Output = PosY<T>;

    fn sub(self, rhs: Height<T>) -> Self::Output {
        PosY::<T>(self.0 - rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position2D<T: Unit> {
    pub x: PosX<T>,
    pub y: PosY<T>,
}

impl<T: Unit + Copy + AddTrait<Output = T> + SubTrait<Output = T> + Ord> Position2D<T> {
    pub fn origin() -> Self {
        Self {
            x: PosX(T::zero()),
            y: PosY(T::zero()),
        }
    }

    pub fn with_x(self, new_x: PosX<T>) -> Self {
        Self {
            x: new_x,
            y: self.y,
        }
    }

    pub fn with_x_of(self, other_position: Self) -> Self {
        self.with_x(other_position.x)
    }

    pub fn with_y(self, new_y: PosY<T>) -> Self {
        Self {
            x: self.x,
            y: new_y,
        }
    }

    pub fn with_y_of(self, other_position: Self) -> Self {
        self.with_y(other_position.y)
    }

    pub fn to_buffer_index(self, width: Width<T>) -> usize {
        let x: usize = self.x.to_primitive().try_into().ok().unwrap();
        let y: usize = self.y.to_primitive().try_into().ok().unwrap();
        let w: usize = width.to_primitive().try_into().ok().unwrap();
        (y * w) + x
    }

    pub fn from_buffer_index(index: usize, width: Width<T>) -> Self {
        Self {
            x: PosX(T::new(index as i32 % width.to_primitive())),
            y: PosY(T::new(index as i32 / width.to_primitive())),
        }
    }

    pub fn to_bracket_geometry_point(self) -> Point {
        Point {
            x: self.x.0.to_primitive(),
            y: self.y.0.to_primitive(),
        }
    }

    pub fn from_bracket_geometry_point(point: Point) -> Self {
        Self {
            x: PosX(T::new(point.x)),
            y: PosY(T::new(point.y)),
        }
    }
}

impl<T: Unit + AddTrait<Output = T>> AddTrait<Size2D<T>> for Position2D<T> {
    type Output = Self;

    fn add(self, rhs: Size2D<T>) -> Self::Output {
        Self {
            x: self.x + rhs.width,
            y: self.y + rhs.height,
        }
    }
}

impl<T: Unit + SubTrait<Output = T>> SubTrait<Position2D<T>> for Position2D<T> {
    type Output = Size2D<T>;

    fn sub(self, rhs: Position2D<T>) -> Size2D<T> {
        Size2D::<T> {
            width: self.x - rhs.x,
            height: self.y - rhs.y,
        }
    }
}

impl<T: Unit + AddTrait<Output = T>> AddTrait<Width<T>> for Position2D<T> {
    type Output = Self;

    fn add(self, rhs: Width<T>) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y,
        }
    }
}

impl<T: Unit + SubTrait<Output = T>> SubTrait<Width<T>> for Position2D<T> {
    type Output = Self;

    fn sub(self, rhs: Width<T>) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y,
        }
    }
}

impl<T: Unit + AddTrait<Output = T>> AddTrait<Height<T>> for Position2D<T> {
    type Output = Self;

    fn add(self, rhs: Height<T>) -> Self::Output {
        Self {
            x: self.x,
            y: self.y + rhs,
        }
    }
}

impl<T: Unit + SubTrait<Output = T>> SubTrait<Height<T>> for Position2D<T> {
    type Output = Self;

    fn sub(self, rhs: Height<T>) -> Self::Output {
        Self {
            x: self.x,
            y: self.y - rhs,
        }
    }
}

impl<T: Unit + SubTrait<Output = T>> SubTrait<Size2D<T>> for Position2D<T> {
    type Output = Position2D<T>;

    fn sub(self, rhs: Size2D<T>) -> Position2D<T> {
        Position2D::<T> {
            x: self.x - rhs.width,
            y: self.y - rhs.height,
        }
    }
}
