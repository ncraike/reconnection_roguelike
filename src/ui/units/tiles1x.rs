extern crate derive_more;
use super::base::{Height, PosX, PosY, Width};
use super::box2d::Box2D;
use super::pixels::Pixels;
use super::point_and_size::{Point2D, Size2D};
use super::text::TextChars;
use super::tiles2x::Tiles2x;
use super::traits::Unit;
use super::utils::{div_ceil, div_floor};
use derive_more::{Add, Div, Mul, Sub};

pub const TILES_1X_WIDTH_IN_PIXELS: i32 = 16;
pub const TILES_1X_HEIGHT_IN_PIXELS: i32 = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Mul, Div)]
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

    fn to_primitive(&self) -> i32 {
        self.0
    }

    fn abs(&self) -> Self {
        Self(self.to_primitive().abs())
    }
}

impl Width<Tiles1x> {
    pub fn from_pixels_floor(pixel_width: Width<Pixels>) -> Self {
        let pixels = pixel_width.0 .0;
        Self(Tiles1x(div_floor(pixels, TILES_1X_WIDTH_IN_PIXELS)))
    }

    pub fn from_pixels_ceil(pixel_width: Width<Pixels>) -> Self {
        let pixels = pixel_width.0 .0;
        Self(Tiles1x(div_ceil(pixels, TILES_1X_WIDTH_IN_PIXELS)))
    }

    pub fn to_pixels(&self) -> Width<Pixels> {
        let quantity = self.0 .0;
        Width(Pixels(quantity * TILES_1X_WIDTH_IN_PIXELS))
    }

    pub fn to_text_chars_floor(&self) -> Width<TextChars> {
        Width::<TextChars>::from_pixels_floor(self.to_pixels())
    }

    pub fn to_text_chars_ceil(&self) -> Width<TextChars> {
        Width::<TextChars>::from_pixels_ceil(self.to_pixels())
    }

    pub fn to_tiles2x_floor(&self) -> Width<Tiles2x> {
        Width::<Tiles2x>::from_pixels_floor(self.to_pixels())
    }

    pub fn to_tiles2x_ceil(&self) -> Width<Tiles2x> {
        Width::<Tiles2x>::from_pixels_ceil(self.to_pixels())
    }
}

impl From<Width<Tiles1x>> for Width<Pixels> {
    fn from(width: Width<Tiles1x>) -> Self {
        width.to_pixels()
    }
}

impl Height<Tiles1x> {
    pub fn from_pixels_floor(pixel_height: Height<Pixels>) -> Self {
        let pixels = pixel_height.0 .0;
        Self(Tiles1x(div_floor(pixels, TILES_1X_HEIGHT_IN_PIXELS)))
    }

    pub fn from_pixels_ceil(pixel_height: Height<Pixels>) -> Self {
        let pixels = pixel_height.0 .0;
        Self(Tiles1x(div_ceil(pixels, TILES_1X_HEIGHT_IN_PIXELS)))
    }

    pub fn to_pixels(&self) -> Height<Pixels> {
        let quantity = self.0 .0;
        Height(Pixels(quantity * TILES_1X_HEIGHT_IN_PIXELS))
    }

    pub fn to_text_chars_floor(&self) -> Height<TextChars> {
        Height::<TextChars>::from_pixels_floor(self.to_pixels())
    }

    pub fn to_text_chars_ceil(&self) -> Height<TextChars> {
        Height::<TextChars>::from_pixels_ceil(self.to_pixels())
    }

    pub fn to_tiles2x_floor(&self) -> Height<Tiles2x> {
        Height::<Tiles2x>::from_pixels_floor(self.to_pixels())
    }

    pub fn to_tiles2x_ceil(&self) -> Height<Tiles2x> {
        Height::<Tiles2x>::from_pixels_ceil(self.to_pixels())
    }
}

impl From<Height<Tiles1x>> for Height<Pixels> {
    fn from(height: Height<Tiles1x>) -> Self {
        height.to_pixels()
    }
}

impl PosX<Tiles1x> {
    pub fn from_pixels_floor(pixels_posx: PosX<Pixels>) -> Self {
        let pixels = pixels_posx.to_primitive();
        Self(Tiles1x(div_floor(pixels, TILES_1X_WIDTH_IN_PIXELS)))
    }

    pub fn from_pixels_ceil(pixels_posx: PosX<Pixels>) -> Self {
        let pixels = pixels_posx.to_primitive();
        Self(Tiles1x(div_ceil(pixels, TILES_1X_WIDTH_IN_PIXELS)))
    }

    pub fn to_pixels(&self) -> PosX<Pixels> {
        let quantity = self.to_primitive();
        PosX(Pixels(quantity * TILES_1X_WIDTH_IN_PIXELS))
    }

    pub fn to_text_chars_floor(&self) -> PosX<TextChars> {
        PosX::<TextChars>::from_pixels_floor(self.to_pixels())
    }

    pub fn to_text_chars_ceil(&self) -> PosX<TextChars> {
        PosX::<TextChars>::from_pixels_ceil(self.to_pixels())
    }

    pub fn to_tiles2x_floor(&self) -> PosX<Tiles2x> {
        PosX::<Tiles2x>::from_pixels_floor(self.to_pixels())
    }

    pub fn to_tiles2x_ceil(&self) -> PosX<Tiles2x> {
        PosX::<Tiles2x>::from_pixels_ceil(self.to_pixels())
    }
}

impl PosY<Tiles1x> {
    pub fn from_pixels_floor(pixels_posy: PosY<Pixels>) -> Self {
        let pixels = pixels_posy.to_primitive();
        Self(Tiles1x(div_floor(pixels, TILES_1X_WIDTH_IN_PIXELS)))
    }

    pub fn from_pixels_ceil(pixel_posy: PosY<Pixels>) -> Self {
        let pixels = pixel_posy.to_primitive();
        Self(Tiles1x(div_ceil(pixels, TILES_1X_WIDTH_IN_PIXELS)))
    }

    pub fn to_pixels(&self) -> PosY<Pixels> {
        let quantity = self.to_primitive();
        PosY(Pixels(quantity * TILES_1X_WIDTH_IN_PIXELS))
    }

    pub fn to_text_chars_floor(&self) -> PosY<TextChars> {
        PosY::<TextChars>::from_pixels_floor(self.to_pixels())
    }

    pub fn to_text_chars_ceil(&self) -> PosY<TextChars> {
        PosY::<TextChars>::from_pixels_ceil(self.to_pixels())
    }

    pub fn to_tiles2x_floor(&self) -> PosY<Tiles2x> {
        PosY::<Tiles2x>::from_pixels_floor(self.to_pixels())
    }

    pub fn to_tiles2x_ceil(&self) -> PosY<Tiles2x> {
        PosY::<Tiles2x>::from_pixels_ceil(self.to_pixels())
    }
}

impl Size2D<Tiles1x> {
    pub fn from_pixels_floor(pixels_size: Size2D<Pixels>) -> Self {
        Size2D::<Tiles1x>::new_from_width_height(
            Width::<Tiles1x>::from_pixels_floor(pixels_size.width),
            Height::<Tiles1x>::from_pixels_floor(pixels_size.height),
        )
    }

    pub fn from_pixels_ceil(pixels_size: Size2D<Pixels>) -> Self {
        Size2D::<Tiles1x>::new_from_width_height(
            Width::<Tiles1x>::from_pixels_ceil(pixels_size.width),
            Height::<Tiles1x>::from_pixels_ceil(pixels_size.height),
        )
    }

    pub fn to_pixels(&self) -> Size2D<Pixels> {
        Size2D::<Pixels>::new_from_width_height(self.width.to_pixels(), self.height.to_pixels())
    }

    pub fn to_text_chars_floor(&self) -> Size2D<TextChars> {
        Size2D::<TextChars>::new_from_width_height(
            Width::<TextChars>::from_pixels_floor(self.width.to_pixels()),
            Height::<TextChars>::from_pixels_floor(self.height.to_pixels()),
        )
    }

    pub fn to_text_chars_ceil(&self) -> Size2D<TextChars> {
        Size2D::<TextChars>::new_from_width_height(
            Width::<TextChars>::from_pixels_ceil(self.width.to_pixels()),
            Height::<TextChars>::from_pixels_ceil(self.height.to_pixels()),
        )
    }

    pub fn to_tiles2x_floor(&self) -> Size2D<Tiles2x> {
        Size2D::<Tiles2x>::new_from_width_height(
            Width::<Tiles2x>::from_pixels_floor(self.width.to_pixels()),
            Height::<Tiles2x>::from_pixels_floor(self.height.to_pixels()),
        )
    }

    pub fn to_tiles2x_ceil(&self) -> Size2D<Tiles2x> {
        Size2D::<Tiles2x>::new_from_width_height(
            Width::<Tiles2x>::from_pixels_ceil(self.width.to_pixels()),
            Height::<Tiles2x>::from_pixels_ceil(self.height.to_pixels()),
        )
    }
}

impl Point2D<Tiles1x> {
    pub fn from_pixels_floor(pixels: Point2D<Pixels>) -> Self {
        Point2D::<Tiles1x>::new_from_x_y(
            PosX::<Tiles1x>::from_pixels_floor(pixels.x),
            PosY::<Tiles1x>::from_pixels_floor(pixels.y),
        )
    }

    pub fn from_pixels_ceil(pixels: Point2D<Pixels>) -> Self {
        Point2D::<Tiles1x>::new_from_x_y(
            PosX::<Tiles1x>::from_pixels_ceil(pixels.x),
            PosY::<Tiles1x>::from_pixels_ceil(pixels.y),
        )
    }

    pub fn to_pixels(&self) -> Point2D<Pixels> {
        Point2D::<Pixels>::new_from_x_y(self.x.to_pixels(), self.y.to_pixels())
    }

    pub fn to_text_chars_floor(&self) -> Point2D<TextChars> {
        Point2D::<TextChars>::new_from_x_y(
            PosX::<TextChars>::from_pixels_floor(self.x.to_pixels()),
            PosY::<TextChars>::from_pixels_floor(self.y.to_pixels()),
        )
    }

    pub fn to_text_chars_ceil(&self) -> Point2D<TextChars> {
        Point2D::<TextChars>::new_from_x_y(
            PosX::<TextChars>::from_pixels_ceil(self.x.to_pixels()),
            PosY::<TextChars>::from_pixels_ceil(self.y.to_pixels()),
        )
    }

    pub fn to_tiles2x_floor(&self) -> Point2D<Tiles2x> {
        Point2D::<Tiles2x>::new_from_x_y(
            PosX::<Tiles2x>::from_pixels_floor(self.x.to_pixels()),
            PosY::<Tiles2x>::from_pixels_floor(self.y.to_pixels()),
        )
    }

    pub fn to_tiles2x_ceil(&self) -> Point2D<Tiles2x> {
        Point2D::<Tiles2x>::new_from_x_y(
            PosX::<Tiles2x>::from_pixels_ceil(self.x.to_pixels()),
            PosY::<Tiles2x>::from_pixels_ceil(self.y.to_pixels()),
        )
    }
}

impl Box2D<Tiles1x> {
    pub fn from_pixels_floor(pixels_box: Box2D<Pixels>) -> Self {
        Box2D::<Tiles1x>::new_from_p1_p2(
            Point2D::<Tiles1x>::from_pixels_floor(pixels_box.p1),
            Point2D::<Tiles1x>::from_pixels_floor(pixels_box.p2),
        )
    }

    pub fn from_pixels_ceil(pixels_box: Box2D<Pixels>) -> Self {
        Box2D::<Tiles1x>::new_from_p1_p2(
            Point2D::<Tiles1x>::from_pixels_ceil(pixels_box.p1),
            Point2D::<Tiles1x>::from_pixels_ceil(pixels_box.p2),
        )
    }

    pub fn to_pixels(&self) -> Box2D<Pixels> {
        Box2D::<Pixels>::new_from_p1_p2(self.p1.to_pixels(), self.p2.to_pixels())
    }

    pub fn to_text_chars_floor(&self) -> Box2D<TextChars> {
        Box2D::<TextChars>::new_from_p1_p2(
            Point2D::<TextChars>::from_pixels_floor(self.p1.to_pixels()),
            Point2D::<TextChars>::from_pixels_floor(self.p2.to_pixels()),
        )
    }

    pub fn to_text_chars_ceil(&self) -> Box2D<TextChars> {
        Box2D::<TextChars>::new_from_p1_p2(
            Point2D::<TextChars>::from_pixels_ceil(self.p1.to_pixels()),
            Point2D::<TextChars>::from_pixels_ceil(self.p2.to_pixels()),
        )
    }

    pub fn to_tiles2x_floor(&self) -> Box2D<Tiles2x> {
        Box2D::<Tiles2x>::new_from_p1_p2(
            Point2D::<Tiles2x>::from_pixels_floor(self.p1.to_pixels()),
            Point2D::<Tiles2x>::from_pixels_floor(self.p2.to_pixels()),
        )
    }

    pub fn to_tiles2x_ceil(&self) -> Box2D<Tiles2x> {
        Box2D::<Tiles2x>::new_from_p1_p2(
            Point2D::<Tiles2x>::from_pixels_ceil(self.p1.to_pixels()),
            Point2D::<Tiles2x>::from_pixels_ceil(self.p2.to_pixels()),
        )
    }
}

pub const ONE_TILE1X: Size2D<Tiles1x> = Size2D::<Tiles1x> {
    width: Width(Tiles1x(1)),
    height: Height(Tiles1x(1)),
};
