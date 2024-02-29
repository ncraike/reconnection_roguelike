use super::{Height, PosX, PosY, Position2D, Size2D, Unit, Width};
use bracket_geometry::prelude::Rect;
use std::cmp::{max, min};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Box2D<T: Unit> {
    pub p1: Position2D<T>,
    pub p2: Position2D<T>,
}

impl<
        T: Unit
            + Copy
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<i32, Output = T>
            + Div<i32, Output = T>
            + Ord,
    > Box2D<T>
{
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

    pub fn center(&self) -> Position2D<T> {
        self.p1 + (self.size() / 2)
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

    pub fn contains(&self, position: Position2D<T>) -> bool {
        self.x1() <= position.x
            && position.x <= self.x2()
            && self.y1() <= position.y
            && position.y <= self.y2()
    }

    pub fn normalize(&self) -> Self {
        Self {
            p1: Position2D::<T> {
                x: min(self.p1.x, self.p2.x),
                y: min(self.p1.y, self.p2.y),
            },
            p2: Position2D::<T> {
                x: max(self.p1.x, self.p2.x),
                y: max(self.p1.y, self.p2.y),
            },
        }
    }

    pub fn split_from_left(&self, offset_from_left: Width<T>) -> (Box2D<T>, Box2D<T>) {
        let right_box = Self {
            p1: self.p1 + offset_from_left,
            p2: self.p2,
        };
        let left_box = Self {
            p1: self.p1,
            p2: self.p2.with_x_of(right_box.p1),
        };
        (left_box, right_box)
    }

    pub fn split_from_right(&self, offset_from_right: Width<T>) -> (Box2D<T>, Box2D<T>) {
        let left_box = Self {
            p1: self.p1,
            p2: self.p2 - offset_from_right,
        };
        let right_box = Self {
            p1: self.p1.with_x_of(left_box.p2),
            p2: self.p2,
        };
        (left_box, right_box)
    }

    pub fn split_from_top(&self, offset_from_top: Height<T>) -> (Box2D<T>, Box2D<T>) {
        let bottom_box = Self {
            p1: self.p1 + offset_from_top,
            p2: self.p2,
        };
        let top_box = Self {
            p1: self.p1,
            p2: self.p2.with_y_of(bottom_box.p1),
        };
        (top_box, bottom_box)
    }

    pub fn split_from_bottom(&self, offset_from_bottom: Height<T>) -> (Box2D<T>, Box2D<T>) {
        let top_box = Self {
            p1: self.p1,
            p2: self.p2 - offset_from_bottom,
        };
        let bottom_box = Self {
            p1: self.p1.with_y_of(top_box.p2),
            p2: self.p2,
        };
        (top_box, bottom_box)
    }

    pub fn to_bracket_geometry_rect(self) -> Rect {
        Rect::with_exact(
            self.x1().0.to_primitive(),
            self.y1().0.to_primitive(),
            self.x2().0.to_primitive(),
            self.y2().0.to_primitive(),
        )
    }

    /// Calls a function for each x/y position in the box
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Position2D<T>),
    {
        for y in self.y1().to_primitive()..self.y2().to_primitive() {
            for x in self.x1().to_primitive()..self.x2().to_primitive() {
                f(Position2D {
                    x: PosX(T::new(x)),
                    y: PosY(T::new(y)),
                });
            }
        }
    }
}
