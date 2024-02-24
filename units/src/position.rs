use bracket_geometry::prelude::Point;
use std::ops::Add as AddTrait;
use std::ops::Sub as SubTrait;
extern crate derive_more;
use super::{HeightI32, Size2DI32, UnitI32, WidthI32};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PosXI32<T: UnitI32>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PosYI32<T: UnitI32>(pub T);

impl<T: UnitI32 + AddTrait<Output = T>> AddTrait<WidthI32<T>> for PosXI32<T> {
    type Output = Self;

    fn add(self, rhs: WidthI32<T>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: UnitI32 + SubTrait<Output = T>> SubTrait<PosXI32<T>> for PosXI32<T> {
    type Output = WidthI32<T>;

    fn sub(self, rhs: PosXI32<T>) -> Self::Output {
        WidthI32::<T>(self.0 - rhs.0)
    }
}

impl<T: UnitI32 + SubTrait<Output = T>> SubTrait<WidthI32<T>> for PosXI32<T> {
    type Output = PosXI32<T>;

    fn sub(self, rhs: WidthI32<T>) -> Self::Output {
        PosXI32::<T>(self.0 - rhs.0)
    }
}

impl<T: UnitI32 + AddTrait<Output = T>> AddTrait<HeightI32<T>> for PosYI32<T> {
    type Output = Self;

    fn add(self, rhs: HeightI32<T>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: UnitI32 + SubTrait<Output = T>> SubTrait<PosYI32<T>> for PosYI32<T> {
    type Output = HeightI32<T>;

    fn sub(self, rhs: PosYI32<T>) -> Self::Output {
        HeightI32::<T>(self.0 - rhs.0)
    }
}

impl<T: UnitI32 + SubTrait<Output = T>> SubTrait<HeightI32<T>> for PosYI32<T> {
    type Output = PosYI32<T>;

    fn sub(self, rhs: HeightI32<T>) -> Self::Output {
        PosYI32::<T>(self.0 - rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position2DI32<T: UnitI32> {
    pub x: PosXI32<T>,
    pub y: PosYI32<T>,
}

impl<T: UnitI32 + Copy + AddTrait<Output = T> + SubTrait<Output = T> + Ord> Position2DI32<T> {
    pub fn origin() -> Self {
        Self {
            x: PosXI32(T::zero()),
            y: PosYI32(T::zero()),
        }
    }

    pub fn with_x(self, new_x: PosXI32<T>) -> Self {
        Self {
            x: new_x,
            y: self.y,
        }
    }

    pub fn with_x_of(self, other_position: Self) -> Self {
        self.with_x(other_position.x)
    }

    pub fn with_y(self, new_y: PosYI32<T>) -> Self {
        Self {
            x: self.x,
            y: new_y,
        }
    }

    pub fn with_y_of(self, other_position: Self) -> Self {
        self.with_y(other_position.y)
    }

    pub fn to_bracket_geometry_point(self) -> Point {
        Point {
            x: self.x.0.to_primitive(),
            y: self.y.0.to_primitive(),
        }
    }
}

impl<T: UnitI32 + AddTrait<Output = T>> AddTrait<Size2DI32<T>> for Position2DI32<T> {
    type Output = Self;

    fn add(self, rhs: Size2DI32<T>) -> Self::Output {
        Self {
            x: self.x + rhs.width,
            y: self.y + rhs.height,
        }
    }
}

impl<T: UnitI32 + SubTrait<Output = T>> SubTrait<Position2DI32<T>> for Position2DI32<T> {
    type Output = Size2DI32<T>;

    fn sub(self, rhs: Position2DI32<T>) -> Size2DI32<T> {
        Size2DI32::<T> {
            width: self.x - rhs.x,
            height: self.y - rhs.y,
        }
    }
}

impl<T: UnitI32 + AddTrait<Output = T>> AddTrait<WidthI32<T>> for Position2DI32<T> {
    type Output = Self;

    fn add(self, rhs: WidthI32<T>) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y,
        }
    }
}

impl<T: UnitI32 + SubTrait<Output = T>> SubTrait<WidthI32<T>> for Position2DI32<T> {
    type Output = Self;

    fn sub(self, rhs: WidthI32<T>) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y,
        }
    }
}

impl<T: UnitI32 + AddTrait<Output = T>> AddTrait<HeightI32<T>> for Position2DI32<T> {
    type Output = Self;

    fn add(self, rhs: HeightI32<T>) -> Self::Output {
        Self {
            x: self.x,
            y: self.y + rhs,
        }
    }
}

impl<T: UnitI32 + SubTrait<Output = T>> SubTrait<HeightI32<T>> for Position2DI32<T> {
    type Output = Self;

    fn sub(self, rhs: HeightI32<T>) -> Self::Output {
        Self {
            x: self.x,
            y: self.y - rhs,
        }
    }
}
