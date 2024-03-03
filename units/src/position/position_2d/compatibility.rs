use super::Position2D;
use crate::{PosX, PosY, Unit};
use bracket_geometry::prelude::Point;

impl<T: Unit> Position2D<T> {
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
