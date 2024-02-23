extern crate derive_more;
use super::pixels::Pixels;
use super::tiles1x::Tiles1x;
use super::tiles2x::Tiles2x;
use derive_more::{Add, Div, Mul, Sub};
use units_proc_macros::{ConvertibleIntegerUnitI32, DerivedIntegerUnitI32};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Add,
    Sub,
    Mul,
    Div,
    DerivedIntegerUnitI32,
    ConvertibleIntegerUnitI32,
)]
#[base_unit(Pixels, 8, 16)]
#[convert_to(Pixels, Tiles1x, Tiles2x)]
pub struct TextChars(pub i32);

// impl TextChars {
//     pub fn new_width(value: i32) -> Width<TextChars> {
//         Width::<TextChars>(TextChars(value))
//     }

//     pub fn new_height(value: i32) -> Height<TextChars> {
//         Height::<TextChars>(TextChars(value))
//     }

//     pub fn new_posx(value: i32) -> PosX<Self> {
//         PosX(Self(value))
//     }

//     pub fn new_posy(value: i32) -> PosY<Self> {
//         PosY(Self(value))
//     }

//     pub fn new_point2d(x: i32, y: i32) -> Point2D<Self> {
//         Point2D::<Self>::new_from_x_y(PosX(Self(x)), PosY(Self(y)))
//     }

//     pub fn new_size2d(width: i32, height: i32) -> Size2D<Self> {
//         Size2D::<Self>::new_from_width_height(Width(Self(width)), Height(Self(height)))
//     }

//     pub fn new_box2d_from_x1_y1_x2_y2(x1: i32, y1: i32, x2: i32, y2: i32) -> Box2D<Self> {
//         Box2D::<Self>::new_from_p1_p2(Self::new_point2d(x1, y1), Self::new_point2d(x2, y2))
//     }

//     pub fn new_box2d_from_width_height(width: i32, height: i32) -> Box2D<Self> {
//         Box2D::<Self>::new_from_size(Self::new_size2d(width, height))
//     }
// }

// impl Unit for TextChars {
//     type ValueType = i32;

//     fn new(value: i32) -> Self {
//         Self(value)
//     }

//     fn zero() -> Self {
//         Self(0)
//     }

//     fn to_primitive(&self) -> i32 {
//         self.0
//     }

//     fn abs(&self) -> Self {
//         Self(self.to_primitive().abs())
//     }
// }

// impl Width<TextChars> {
//     pub fn from_pixels_floor(pixel_width: Width<Pixels>) -> Self {
//         let pixels = pixel_width.0 .0;
//         Self(TextChars(div_floor(pixels, TEXT_CHARS_WIDTH_IN_PIXELS)))
//     }

//     pub fn from_pixels_ceil(pixel_width: Width<Pixels>) -> Self {
//         let pixels = pixel_width.0 .0;
//         Self(TextChars(div_ceil(pixels, TEXT_CHARS_WIDTH_IN_PIXELS)))
//     }

//     pub fn to_pixels(&self) -> Width<Pixels> {
//         let quantity = self.0 .0;
//         Width(Pixels(quantity * TEXT_CHARS_WIDTH_IN_PIXELS))
//     }

//     pub fn to_tiles1x_floor(&self) -> Width<Tiles1x> {
//         Width::<Tiles1x>::from_pixels_floor(self.to_pixels())
//     }

//     pub fn to_tiles1x_ceil(&self) -> Width<Tiles1x> {
//         Width::<Tiles1x>::from_pixels_ceil(self.to_pixels())
//     }

//     pub fn to_tiles2x_floor(&self) -> Width<Tiles2x> {
//         Width::<Tiles2x>::from_pixels_floor(self.to_pixels())
//     }

//     pub fn to_tiles2x_ceil(&self) -> Width<Tiles2x> {
//         Width::<Tiles2x>::from_pixels_ceil(self.to_pixels())
//     }
// }

// impl From<Width<TextChars>> for Width<Pixels> {
//     fn from(width: Width<TextChars>) -> Self {
//         width.to_pixels()
//     }
// }

// impl Height<TextChars> {
//     pub fn from_pixels_floor(pixel_height: Height<Pixels>) -> Self {
//         let pixels = pixel_height.0 .0;
//         Self(TextChars(div_floor(pixels, TEXT_CHARS_HEIGHT_IN_PIXELS)))
//     }

//     pub fn from_pixels_ceil(pixel_height: Height<Pixels>) -> Self {
//         let pixels = pixel_height.0 .0;
//         Self(TextChars(div_ceil(pixels, TEXT_CHARS_HEIGHT_IN_PIXELS)))
//     }

//     pub fn to_pixels(&self) -> Height<Pixels> {
//         let quantity = self.0 .0;
//         Height(Pixels(quantity * TEXT_CHARS_HEIGHT_IN_PIXELS))
//     }

//     pub fn to_tiles1x_floor(&self) -> Height<Tiles1x> {
//         Height::<Tiles1x>::from_pixels_floor(self.to_pixels())
//     }

//     pub fn to_tiles1x_ceil(&self) -> Height<Tiles1x> {
//         Height::<Tiles1x>::from_pixels_ceil(self.to_pixels())
//     }

//     pub fn to_tiles2x_floor(&self) -> Height<Tiles2x> {
//         Height::<Tiles2x>::from_pixels_floor(self.to_pixels())
//     }

//     pub fn to_tiles2x_ceil(&self) -> Height<Tiles2x> {
//         Height::<Tiles2x>::from_pixels_ceil(self.to_pixels())
//     }
// }

// impl From<Height<TextChars>> for Height<Pixels> {
//     fn from(height: Height<TextChars>) -> Self {
//         height.to_pixels()
//     }
// }

// impl PosX<TextChars> {
//     pub fn from_pixels_floor(pixels_posx: PosX<Pixels>) -> Self {
//         let pixels = pixels_posx.to_primitive();
//         Self(TextChars(div_floor(pixels, TEXT_CHARS_WIDTH_IN_PIXELS)))
//     }

//     pub fn from_pixels_ceil(pixels_posx: PosX<Pixels>) -> Self {
//         let pixels = pixels_posx.to_primitive();
//         Self(TextChars(div_ceil(pixels, TEXT_CHARS_WIDTH_IN_PIXELS)))
//     }

//     pub fn to_pixels(&self) -> PosX<Pixels> {
//         let quantity = self.to_primitive();
//         PosX(Pixels(quantity * TEXT_CHARS_WIDTH_IN_PIXELS))
//     }

//     pub fn to_tiles1x_floor(&self) -> PosX<Tiles1x> {
//         PosX::<Tiles1x>::from_pixels_floor(self.to_pixels())
//     }

//     pub fn to_tiles1x_ceil(&self) -> PosX<Tiles1x> {
//         PosX::<Tiles1x>::from_pixels_ceil(self.to_pixels())
//     }

//     pub fn to_tiles2x_floor(&self) -> PosX<Tiles2x> {
//         PosX::<Tiles2x>::from_pixels_floor(self.to_pixels())
//     }

//     pub fn to_tiles2x_ceil(&self) -> PosX<Tiles2x> {
//         PosX::<Tiles2x>::from_pixels_ceil(self.to_pixels())
//     }
// }

// impl PosY<TextChars> {
//     pub fn from_pixels_floor(pixels_posy: PosY<Pixels>) -> Self {
//         let pixels = pixels_posy.to_primitive();
//         Self(TextChars(div_floor(pixels, TEXT_CHARS_WIDTH_IN_PIXELS)))
//     }

//     pub fn from_pixels_ceil(pixels_posy: PosY<Pixels>) -> Self {
//         let pixels = pixels_posy.to_primitive();
//         Self(TextChars(div_ceil(pixels, TEXT_CHARS_WIDTH_IN_PIXELS)))
//     }

//     pub fn to_pixels(&self) -> PosY<Pixels> {
//         let quantity = self.to_primitive();
//         PosY(Pixels(quantity * TEXT_CHARS_WIDTH_IN_PIXELS))
//     }

//     pub fn to_tiles1x_floor(&self) -> PosY<Tiles1x> {
//         PosY::<Tiles1x>::from_pixels_floor(self.to_pixels())
//     }

//     pub fn to_tiles1x_ceil(&self) -> PosY<Tiles1x> {
//         PosY::<Tiles1x>::from_pixels_ceil(self.to_pixels())
//     }

//     pub fn to_tiles2x_floor(&self) -> PosY<Tiles2x> {
//         PosY::<Tiles2x>::from_pixels_floor(self.to_pixels())
//     }

//     pub fn to_tiles2x_ceil(&self) -> PosY<Tiles2x> {
//         PosY::<Tiles2x>::from_pixels_ceil(self.to_pixels())
//     }
// }

// impl Size2D<TextChars> {
//     pub fn from_pixels_floor(pixels_size: Size2D<Pixels>) -> Self {
//         Size2D::<TextChars>::new_from_width_height(
//             Width::<TextChars>::from_pixels_floor(pixels_size.width),
//             Height::<TextChars>::from_pixels_floor(pixels_size.height),
//         )
//     }

//     pub fn from_pixels_ceil(pixels_size: Size2D<Pixels>) -> Self {
//         Size2D::<TextChars>::new_from_width_height(
//             Width::<TextChars>::from_pixels_ceil(pixels_size.width),
//             Height::<TextChars>::from_pixels_ceil(pixels_size.height),
//         )
//     }

//     pub fn to_pixels(&self) -> Size2D<Pixels> {
//         Size2D::<Pixels>::new_from_width_height(self.width.to_pixels(), self.height.to_pixels())
//     }

//     pub fn to_tiles1x_floor(&self) -> Size2D<Tiles1x> {
//         Size2D::<Tiles1x>::new_from_width_height(
//             Width::<Tiles1x>::from_pixels_floor(self.width.to_pixels()),
//             Height::<Tiles1x>::from_pixels_floor(self.height.to_pixels()),
//         )
//     }

//     pub fn to_tiles1x_ceil(&self) -> Size2D<Tiles1x> {
//         Size2D::<Tiles1x>::new_from_width_height(
//             Width::<Tiles1x>::from_pixels_ceil(self.width.to_pixels()),
//             Height::<Tiles1x>::from_pixels_ceil(self.height.to_pixels()),
//         )
//     }

//     pub fn to_tiles2x_floor(&self) -> Size2D<Tiles2x> {
//         Size2D::<Tiles2x>::new_from_width_height(
//             Width::<Tiles2x>::from_pixels_floor(self.width.to_pixels()),
//             Height::<Tiles2x>::from_pixels_floor(self.height.to_pixels()),
//         )
//     }

//     pub fn to_tiles2x_ceil(&self) -> Size2D<Tiles2x> {
//         Size2D::<Tiles2x>::new_from_width_height(
//             Width::<Tiles2x>::from_pixels_ceil(self.width.to_pixels()),
//             Height::<Tiles2x>::from_pixels_ceil(self.height.to_pixels()),
//         )
//     }
// }

// impl Point2D<TextChars> {
//     pub fn from_pixels_floor(pixels: Point2D<Pixels>) -> Self {
//         Point2D::<TextChars>::new_from_x_y(
//             PosX::<TextChars>::from_pixels_floor(pixels.x),
//             PosY::<TextChars>::from_pixels_floor(pixels.y),
//         )
//     }

//     pub fn from_pixels_ceil(pixels: Point2D<Pixels>) -> Self {
//         Point2D::<TextChars>::new_from_x_y(
//             PosX::<TextChars>::from_pixels_ceil(pixels.x),
//             PosY::<TextChars>::from_pixels_ceil(pixels.y),
//         )
//     }

//     pub fn to_pixels(&self) -> Point2D<Pixels> {
//         Point2D::<Pixels>::new_from_x_y(self.x.to_pixels(), self.y.to_pixels())
//     }

//     pub fn to_tiles1x_floor(&self) -> Point2D<Tiles1x> {
//         Point2D::<Tiles1x>::new_from_x_y(
//             PosX::<Tiles1x>::from_pixels_floor(self.x.to_pixels()),
//             PosY::<Tiles1x>::from_pixels_floor(self.y.to_pixels()),
//         )
//     }

//     pub fn to_tiles1x_ceil(&self) -> Point2D<Tiles1x> {
//         Point2D::<Tiles1x>::new_from_x_y(
//             PosX::<Tiles1x>::from_pixels_ceil(self.x.to_pixels()),
//             PosY::<Tiles1x>::from_pixels_ceil(self.y.to_pixels()),
//         )
//     }

//     pub fn to_tiles2x_floor(&self) -> Point2D<Tiles2x> {
//         Point2D::<Tiles2x>::new_from_x_y(
//             PosX::<Tiles2x>::from_pixels_floor(self.x.to_pixels()),
//             PosY::<Tiles2x>::from_pixels_floor(self.y.to_pixels()),
//         )
//     }

//     pub fn to_tiles2x_ceil(&self) -> Point2D<Tiles2x> {
//         Point2D::<Tiles2x>::new_from_x_y(
//             PosX::<Tiles2x>::from_pixels_ceil(self.x.to_pixels()),
//             PosY::<Tiles2x>::from_pixels_ceil(self.y.to_pixels()),
//         )
//     }
// }

// impl Box2D<TextChars> {
//     pub fn from_pixels_floor(pixels_box: Box2D<Pixels>) -> Self {
//         Box2D::<TextChars>::new_from_p1_p2(
//             Point2D::<TextChars>::from_pixels_floor(pixels_box.p1),
//             Point2D::<TextChars>::from_pixels_floor(pixels_box.p2),
//         )
//     }

//     pub fn from_pixels_ceil(pixels_box: Box2D<Pixels>) -> Self {
//         Box2D::<TextChars>::new_from_p1_p2(
//             Point2D::<TextChars>::from_pixels_ceil(pixels_box.p1),
//             Point2D::<TextChars>::from_pixels_ceil(pixels_box.p2),
//         )
//     }

//     pub fn to_pixels(&self) -> Box2D<Pixels> {
//         Box2D::<Pixels>::new_from_p1_p2(self.p1.to_pixels(), self.p2.to_pixels())
//     }

//     pub fn to_tiles1x_floor(&self) -> Box2D<Tiles1x> {
//         Box2D::<Tiles1x>::new_from_p1_p2(
//             Point2D::<Tiles1x>::from_pixels_floor(self.p1.to_pixels()),
//             Point2D::<Tiles1x>::from_pixels_floor(self.p2.to_pixels()),
//         )
//     }

//     pub fn to_tiles1x_ceil(&self) -> Box2D<Tiles1x> {
//         Box2D::<Tiles1x>::new_from_p1_p2(
//             Point2D::<Tiles1x>::from_pixels_ceil(self.p1.to_pixels()),
//             Point2D::<Tiles1x>::from_pixels_ceil(self.p2.to_pixels()),
//         )
//     }

//     pub fn to_tiles2x_floor(&self) -> Box2D<Tiles2x> {
//         Box2D::<Tiles2x>::new_from_p1_p2(
//             Point2D::<Tiles2x>::from_pixels_floor(self.p1.to_pixels()),
//             Point2D::<Tiles2x>::from_pixels_floor(self.p2.to_pixels()),
//         )
//     }

//     pub fn to_tiles2x_ceil(&self) -> Box2D<Tiles2x> {
//         Box2D::<Tiles2x>::new_from_p1_p2(
//             Point2D::<Tiles2x>::from_pixels_ceil(self.p1.to_pixels()),
//             Point2D::<Tiles2x>::from_pixels_ceil(self.p2.to_pixels()),
//         )
//     }
// }

// pub const ONE_TEXT_CHAR: Size2D<TextChars> = Size2D::<TextChars> {
//     width: Width(TextChars(1)),
//     height: Height(TextChars(1)),
// };
