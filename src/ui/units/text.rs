extern crate derive_more;
use super::base::{Height, PosX, PosY, Width};
use super::box2d::Box2D;
use super::pixels::Pixels;
use super::point_and_size::{Point2D, Size2D};
use super::tiles1x::Tiles1x;
use super::tiles2x::Tiles2x;
use super::traits::Unit;
use super::utils::{div_ceil, div_floor};
use derive_more::{Add, Mul, Sub};

const TEXT_CHARS_WIDTH_IN_PIXELS: i32 = 8;
const TEXT_CHARS_HEIGHT_IN_PIXELS: i32 = 16;

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

    fn to_primitive(&self) -> i32 {
        self.0
    }

    fn abs(&self) -> Self {
        Self(self.to_primitive().abs())
    }
}

impl Width<TextChars> {
    pub fn from_pixels_floor(pixel_width: Width<Pixels>) -> Self {
        let pixels = pixel_width.0 .0;
        Self(TextChars(div_floor(pixels, TEXT_CHARS_WIDTH_IN_PIXELS)))
    }

    pub fn from_pixels_ceil(pixel_width: Width<Pixels>) -> Self {
        let pixels = pixel_width.0 .0;
        Self(TextChars(div_ceil(pixels, TEXT_CHARS_WIDTH_IN_PIXELS)))
    }

    pub fn to_pixels(&self) -> Width<Pixels> {
        let quantity = self.0 .0;
        Width(Pixels(quantity * TEXT_CHARS_WIDTH_IN_PIXELS))
    }

    pub fn to_tiles1x_floor(&self) -> Width<Tiles1x> {
        Width::<Tiles1x>::from_pixels_floor(self.to_pixels())
    }

    pub fn to_tiles1x_ceil(&self) -> Width<Tiles1x> {
        Width::<Tiles1x>::from_pixels_ceil(self.to_pixels())
    }

    pub fn to_tiles2x_floor(&self) -> Width<Tiles2x> {
        Width::<Tiles2x>::from_pixels_floor(self.to_pixels())
    }

    pub fn to_tiles2x_ceil(&self) -> Width<Tiles2x> {
        Width::<Tiles2x>::from_pixels_ceil(self.to_pixels())
    }
}

impl From<Width<TextChars>> for Width<Pixels> {
    fn from(width: Width<TextChars>) -> Self {
        width.to_pixels()
    }
}

impl Height<TextChars> {
    pub fn from_pixels_floor(pixel_height: Height<Pixels>) -> Self {
        let pixels = pixel_height.0 .0;
        Self(TextChars(div_floor(pixels, TEXT_CHARS_HEIGHT_IN_PIXELS)))
    }

    pub fn from_pixels_ceil(pixel_height: Height<Pixels>) -> Self {
        let pixels = pixel_height.0 .0;
        Self(TextChars(div_ceil(pixels, TEXT_CHARS_HEIGHT_IN_PIXELS)))
    }

    pub fn to_pixels(&self) -> Height<Pixels> {
        let quantity = self.0 .0;
        Height(Pixels(quantity * TEXT_CHARS_HEIGHT_IN_PIXELS))
    }

    pub fn to_tiles1x_floor(&self) -> Height<Tiles1x> {
        Height::<Tiles1x>::from_pixels_floor(self.to_pixels())
    }

    pub fn to_tiles1x_ceil(&self) -> Height<Tiles1x> {
        Height::<Tiles1x>::from_pixels_ceil(self.to_pixels())
    }

    pub fn to_tiles2x_floor(&self) -> Height<Tiles2x> {
        Height::<Tiles2x>::from_pixels_floor(self.to_pixels())
    }

    pub fn to_tiles2x_ceil(&self) -> Height<Tiles2x> {
        Height::<Tiles2x>::from_pixels_ceil(self.to_pixels())
    }
}

impl From<Height<TextChars>> for Height<Pixels> {
    fn from(height: Height<TextChars>) -> Self {
        height.to_pixels()
    }
}
