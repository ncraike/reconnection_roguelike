use super::Box2D;
use crate::Unit;
use bracket_geometry::prelude::Rect;
use std::ops::{Add, Sub};

impl<T: Unit + Copy + Add<Output = T> + Sub<Output = T> + Ord> Box2D<T> {
    pub fn to_bracket_geometry_rect(self) -> Rect {
        Rect::with_exact(
            self.x1().0.to_primitive(),
            self.y1().0.to_primitive(),
            self.x2().0.to_primitive(),
            self.y2().0.to_primitive(),
        )
    }
}
