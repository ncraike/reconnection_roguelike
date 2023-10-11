extern crate derive_more;
use super::base::{Height, PosX, PosY, Width};
use super::box2d::Box2D;
use super::pixels::Pixels;
use super::point_and_size::{Point2D, Size2D};
use super::traits::Unit;
use derive_more::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Mul, Sub)]
pub struct TextChars(pub i32);

impl TextChars {
    pub fn new_width(value: i32) -> Width<TextChars> {
        Width::<TextChars>(TextChars(value))
    }

    pub fn new_height(value: i32) -> Height<TextChars> {
        Height::<TextChars>(TextChars(value))
    }

    pub fn new_posx(value: i32) -> PosX<Self> {
        PosX(Self(value))
    }

    pub fn new_posy(value: i32) -> PosY<Self> {
        PosY(Self(value))
    }

    pub fn new_point2d(x: i32, y: i32) -> Point2D<Self> {
        Point2D::<Self>::new_from_x_y(PosX(Self(x)), PosY(Self(y)))
    }

    pub fn new_size2d(width: i32, height: i32) -> Size2D<Self> {
        Size2D::<Self>::new_from_width_height(Width(Self(width)), Height(Self(height)))
    }

    pub fn new_box2d_from_x1_y1_x2_y2(x1: i32, y1: i32, x2: i32, y2: i32) -> Box2D<Self> {
        Box2D::<Self>::new_from_p1_p2(Self::new_point2d(x1, y1), Self::new_point2d(x2, y2))
    }

    pub fn new_box2d_from_width_height(width: i32, height: i32) -> Box2D<Self> {
        Box2D::<Self>::new_from_size(Self::new_size2d(width, height))
    }
}

impl Unit for TextChars {
    type ValueType = i32;

    fn new(value: i32) -> Self {
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

impl From<Width<TextChars>> for Pixels {
    fn from(width: Width<TextChars>) -> Self {
        let quantity = width.0 .0;
        Pixels(quantity * 8)
    }
}

impl From<Height<TextChars>> for Pixels {
    fn from(height: Height<TextChars>) -> Self {
        let quantity = height.0 .0;
        Pixels(quantity * 16)
    }
}
