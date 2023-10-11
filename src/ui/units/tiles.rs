extern crate derive_more;
use super::base::{Height, PosX, PosY, Width};
use super::box2d::Box2D;
use super::pixels::Pixels;
use super::point_and_size::{Point2D, Size2D};
use super::text::TextChars;
use super::traits::Unit;
use super::utils::{div_ceil, div_floor};
use derive_more::{Add, Mul, Sub};

const TILES_1X_WIDTH_IN_PIXELS: i32 = 16;
const TILES_1X_HEIGHT_IN_PIXELS: i32 = 24;
const TILES_2X_WIDTH_IN_PIXELS: i32 = TILES_1X_WIDTH_IN_PIXELS * 2;
const TILES_2X_HEIGHT_IN_PIXELS: i32 = TILES_1X_HEIGHT_IN_PIXELS * 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Mul)]
pub struct Tiles1x(pub i32);

impl Tiles1x {
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

impl Unit for Tiles1x {
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

impl Width<Tiles1x> {
    pub fn to_pixel_width(&self) -> Width<Pixels> {
        let quantity = self.0 .0;
        Width(Pixels(quantity * TILES_1X_WIDTH_IN_PIXELS))
    }

    pub fn from_pixel_width_floor(pixel_width: Width<Pixels>) -> Self {
        let pixels = pixel_width.0 .0;
        Self(Tiles1x(div_floor(pixels, TILES_1X_WIDTH_IN_PIXELS)))
    }

    pub fn from_pixel_width_ceil(pixel_width: Width<Pixels>) -> Self {
        let pixels = pixel_width.0 .0;
        Self(Tiles1x(div_ceil(pixels, TILES_1X_WIDTH_IN_PIXELS)))
    }

    pub fn from_text_chars_floor(text_chars_width: Width<TextChars>) -> Self {
        Self::from_pixel_width_floor(text_chars_width.to_pixel_width())
    }

    pub fn from_text_chars_ceil(text_chars_width: Width<TextChars>) -> Self {
        Self::from_pixel_width_ceil(text_chars_width.to_pixel_width())
    }
}

impl From<Width<Tiles1x>> for Width<Pixels> {
    fn from(width: Width<Tiles1x>) -> Self {
        width.to_pixel_width()
    }
}

impl Height<Tiles1x> {
    pub fn to_pixel_height(&self) -> Height<Pixels> {
        let quantity = self.0 .0;
        Height(Pixels(quantity * TILES_1X_HEIGHT_IN_PIXELS))
    }

    pub fn from_pixel_height_floor(pixel_height: Height<Pixels>) -> Self {
        let pixels = pixel_height.0 .0;
        Self(Tiles1x(div_floor(pixels, TILES_1X_HEIGHT_IN_PIXELS)))
    }

    pub fn from_pixel_height_ceil(pixel_height: Height<Pixels>) -> Self {
        let pixels = pixel_height.0 .0;
        Self(Tiles1x(div_ceil(pixels, TILES_1X_HEIGHT_IN_PIXELS)))
    }

    pub fn from_text_chars_floor(text_chars_height: Height<TextChars>) -> Self {
        Self::from_pixel_height_floor(text_chars_height.to_pixel_height())
    }

    pub fn from_text_chars_ceil(text_chars_height: Height<TextChars>) -> Self {
        Self::from_pixel_height_ceil(text_chars_height.to_pixel_height())
    }
}

impl From<Height<Tiles1x>> for Height<Pixels> {
    fn from(height: Height<Tiles1x>) -> Self {
        let quantity = height.0 .0;
        Height(Pixels(quantity * 24))
    }
}

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

    fn value(&self) -> i32 {
        self.0
    }

    fn abs(&self) -> Self {
        Self(self.value().abs())
    }
}

impl Width<Tiles2x> {
    pub fn to_pixel_width(&self) -> Width<Pixels> {
        let quantity = self.0 .0;
        Width(Pixels(quantity * TILES_2X_WIDTH_IN_PIXELS))
    }

    pub fn from_pixel_width_floor(pixel_width: Width<Pixels>) -> Self {
        let pixels = pixel_width.0 .0;
        Self(Tiles2x(div_floor(pixels, TILES_2X_WIDTH_IN_PIXELS)))
    }

    pub fn from_pixel_width_ceil(pixel_width: Width<Pixels>) -> Self {
        let pixels = pixel_width.0 .0;
        Self(Tiles2x(div_ceil(pixels, TILES_2X_WIDTH_IN_PIXELS)))
    }

    pub fn from_text_chars_floor(text_chars_width: Width<TextChars>) -> Self {
        Self::from_pixel_width_floor(text_chars_width.to_pixel_width())
    }

    pub fn from_text_chars_ceil(text_chars_width: Width<TextChars>) -> Self {
        Self::from_pixel_width_ceil(text_chars_width.to_pixel_width())
    }
}

impl From<Width<Tiles2x>> for Width<Pixels> {
    fn from(width: Width<Tiles2x>) -> Self {
        width.to_pixel_width()
    }
}

impl Height<Tiles2x> {
    pub fn to_pixel_height(&self) -> Height<Pixels> {
        let quantity = self.0 .0;
        Height(Pixels(quantity * TILES_1X_HEIGHT_IN_PIXELS))
    }

    pub fn from_pixel_height_floor(pixel_height: Height<Pixels>) -> Self {
        let pixels = pixel_height.0 .0;
        Self(Tiles2x(div_floor(pixels, TILES_1X_HEIGHT_IN_PIXELS)))
    }

    pub fn from_pixel_height_ceil(pixel_height: Height<Pixels>) -> Self {
        let pixels = pixel_height.0 .0;
        Self(Tiles2x(div_ceil(pixels, TILES_1X_HEIGHT_IN_PIXELS)))
    }

    pub fn from_text_chars_floor(text_chars_height: Height<TextChars>) -> Self {
        Self::from_pixel_height_floor(text_chars_height.to_pixel_height())
    }

    pub fn from_text_chars_ceil(text_chars_height: Height<TextChars>) -> Self {
        Self::from_pixel_height_ceil(text_chars_height.to_pixel_height())
    }
}

impl From<Height<Tiles2x>> for Height<Pixels> {
    fn from(height: Height<Tiles2x>) -> Self {
        height.to_pixel_height()
    }
}
