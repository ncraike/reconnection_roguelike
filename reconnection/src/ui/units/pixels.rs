extern crate derive_more;
extern crate units_proc_macros;
use derive_more::{Add, Mul, Sub};
use units_proc_macros::IntegerUnitI32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Mul, IntegerUnitI32)]
pub struct Pixels(pub i32);

// impl Pixels {
//     pub fn new_width(value: i32) -> Width<Self> {
//         Width(Self(value))
//     }

//     pub fn new_height(value: i32) -> Height<Self> {
//         Height(Self(value))
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
