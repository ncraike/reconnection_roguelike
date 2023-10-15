extern crate derive_more;
use super::base::{Height, PosX, PosY, Width};
use super::box2d::Box2D;
use super::pixels::Pixels;
use super::point_and_size::{Point2D, Size2D};
use super::text::TextChars;
use super::tiles1x::{Tiles1x, TILES_1X_HEIGHT_IN_PIXELS, TILES_1X_WIDTH_IN_PIXELS};
use super::traits::Unit;
use super::utils::{div_ceil, div_floor};
use derive_more::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Mul)]
pub struct Tiles2x(pub i32);

impl Tiles2x {
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

impl Unit for Tiles2x {
    type ValueType = i32;

    fn new(value: i32) -> Self {
        Self(value)
    }

    fn zero() -> Self {
        Self(0)
    }

    fn to_primitive(&self) -> i32 {
        self.0
    }

    fn abs(&self) -> Self {
        Self(self.to_primitive().abs())
    }
}

impl Width<Tiles2x> {
    pub fn from_pixels_floor(pixel_width: Width<Pixels>) -> Self {
        let pixels = pixel_width.0 .0;
        Self(Tiles2x(div_floor(pixels, TILES_1X_WIDTH_IN_PIXELS * 2)))
    }

    pub fn from_pixels_ceil(pixel_width: Width<Pixels>) -> Self {
        let pixels = pixel_width.0 .0;
        Self(Tiles2x(div_ceil(pixels, TILES_1X_WIDTH_IN_PIXELS * 2)))
    }

    pub fn to_pixels(&self) -> Width<Pixels> {
        let quantity = self.0 .0;
        Width(Pixels(quantity * TILES_1X_WIDTH_IN_PIXELS * 2))
    }

    pub fn to_text_chars_floor(&self) -> Width<TextChars> {
        Width::<TextChars>::from_pixels_floor(self.to_pixels())
    }

    pub fn to_text_chars_ceil(&self) -> Width<TextChars> {
        Width::<TextChars>::from_pixels_ceil(self.to_pixels())
    }

    pub fn to_tiles1x(&self) -> Width<Tiles1x> {
        Width::<Tiles1x>::from_pixels_floor(self.to_pixels())
    }
}

impl From<Width<Tiles2x>> for Width<Pixels> {
    fn from(width: Width<Tiles2x>) -> Self {
        width.to_pixels()
    }
}

impl Height<Tiles2x> {
    pub fn from_pixels_floor(pixel_height: Height<Pixels>) -> Self {
        let pixels = pixel_height.0 .0;
        Self(Tiles2x(div_floor(pixels, TILES_1X_HEIGHT_IN_PIXELS * 2)))
    }

    pub fn from_pixels_ceil(pixel_height: Height<Pixels>) -> Self {
        let pixels = pixel_height.0 .0;
        Self(Tiles2x(div_ceil(pixels, TILES_1X_HEIGHT_IN_PIXELS * 2)))
    }

    pub fn to_pixels(&self) -> Height<Pixels> {
        let quantity = self.0 .0;
        Height(Pixels(quantity * TILES_1X_HEIGHT_IN_PIXELS * 2))
    }

    pub fn to_text_chars_floor(&self) -> Height<TextChars> {
        Height::<TextChars>::from_pixels_floor(self.to_pixels())
    }

    pub fn to_text_chars_ceil(&self) -> Height<TextChars> {
        Height::<TextChars>::from_pixels_ceil(self.to_pixels())
    }

    pub fn to_tiles1x(&self) -> Height<Tiles1x> {
        Height::<Tiles1x>::from_pixels_floor(self.to_pixels())
    }
}

impl From<Height<Tiles2x>> for Height<Pixels> {
    fn from(height: Height<Tiles2x>) -> Self {
        height.to_pixels()
    }
}
