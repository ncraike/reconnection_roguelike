use super::{HeightI32, PosXI32, PosYI32, Position2DI32, Size2DI32, UnitI32, WidthI32};
use std::cmp::{max, min};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Box2DI32<T: UnitI32> {
    pub p1: Position2DI32<T>,
    pub p2: Position2DI32<T>,
}

impl<T: UnitI32 + Copy + Add<Output = T> + Sub<Output = T> + Ord> Box2DI32<T> {
    pub fn x1(&self) -> PosXI32<T> {
        self.p1.x
    }

    pub fn y1(&self) -> PosYI32<T> {
        self.p1.y
    }

    pub fn x2(&self) -> PosXI32<T> {
        self.p2.x
    }

    pub fn y2(&self) -> PosYI32<T> {
        self.p2.y
    }

    pub fn size(&self) -> Size2DI32<T> {
        self.p2 - self.p1
    }

    pub fn width(&self) -> WidthI32<T> {
        self.size().width
    }

    pub fn height(&self) -> HeightI32<T> {
        self.size().height
    }

    pub fn normalize(&self) -> Self {
        Self {
            p1: Position2DI32::<T> {
                x: min(self.p1.x, self.p2.x),
                y: min(self.p1.y, self.p2.y),
            },
            p2: Position2DI32::<T> {
                x: max(self.p1.x, self.p2.x),
                y: max(self.p1.y, self.p2.y),
            },
        }
    }

    pub fn split_from_left(&self, offset_from_left: WidthI32<T>) -> (Box2DI32<T>, Box2DI32<T>) {
        let left_box = Self {
            p1: self.p1,
            p2: Position2DI32::<T> {
                x: self.p1.x + offset_from_left,
                y: self.p2.y,
            },
        };
        let right_box = Self {
            p1: self.p1 + offset_from_left,
            p2: self.p2,
        };
        (left_box, right_box)
    }

    pub fn split_from_right(&self, offset_from_right: WidthI32<T>) -> (Box2DI32<T>, Box2DI32<T>) {
        let left_box = Self {
            p1: self.p1,
            p2: self.p2 - offset_from_right,
        };
        let right_box = Self {
            p1: Position2DI32::<T> {
                x: self.p2.x - offset_from_right,
                y: self.p1.y,
            },
            p2: self.p2,
        };
        (left_box, right_box)
    }

    pub fn split_from_top(&self, offset_from_top: HeightI32<T>) -> (Box2DI32<T>, Box2DI32<T>) {
        let top_box = Self {
            p1: self.p1,
            p2: Position2DI32::<T> {
                x: self.p2.x,
                y: self.p1.y + offset_from_top,
            },
        };
        let bottom_box = Self {
            p1: self.p1 + offset_from_top,
            p2: self.p2,
        };
        (top_box, bottom_box)
    }

    pub fn split_from_bottom(
        &self,
        offset_from_bottom: HeightI32<T>,
    ) -> (Box2DI32<T>, Box2DI32<T>) {
        let top_box = Self {
            p1: self.p1,
            p2: self.p2 - offset_from_bottom,
        };
        let bottom_box = Self {
            p1: Position2DI32::<T> {
                x: self.p1.x,
                y: self.p2.y - offset_from_bottom,
            },
            p2: self.p2,
        };
        (top_box, bottom_box)
    }
}
