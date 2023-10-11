extern crate derive_more;
use super::base::{Height, PosX, PosY, Width};
use super::point_and_size::{Point2D, Size2D};
use super::traits::Unit;
use super::Box2D;
use derive_more::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Mul)]
pub struct Pixels(pub i32);

impl Unit for Pixels {
    type ValueType = i32;

    fn new(value: Self::ValueType) -> Self {
        Self(value)
    }

    fn zero() -> Self {
        Self(0)
    }

    fn value(&self) -> i32 {
        self.0
    }

    fn abs(&self) -> Self {
        Self(self.value().abs())
    }
}

impl Pixels {
    pub fn new_width(value: i32) -> Width<Self> {
        Width(Self(value))
    }

    pub fn new_height(value: i32) -> Height<Self> {
        Height(Self(value))
    }

    pub fn new_posx(value: i32) -> PosX<Self> {
        PosX(Self(value))
    }

    pub fn new_posy(value: i32) -> PosY<Self> {
        PosY(Self(value))
    }

    pub fn new_point2d(x: i32, y: i32) -> Point2D<Self> {
        Point2D::<Self>::new_from_x_y(Self(x), Self(y))
    }

    pub fn new_size2d(width: i32, height: i32) -> Size2D<Self> {
        Size2D::<Self>::new_from_width_height(Self(width), Self(height))
    }

    pub fn new_box2d_from_x1_y1_x2_y2(x1: i32, y1: i32, x2: i32, y2: i32) -> Box2D<Self> {
        Box2D::<Self>::new_from_x1_y1_x2_y2(Self(x1), Self(y1), Self(x2), Self(y2))
    }

    pub fn new_box2d_from_width_height(width: i32, height: i32) -> Box2D<Self> {
        Box2D::<Self>::new_from_size(Self::new_size2d(width, height))
    }
}
