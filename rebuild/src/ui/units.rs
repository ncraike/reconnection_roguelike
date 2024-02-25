use bracket_geometry::prelude::Point;
use derive_more::{Add, Div, Mul, Sub};
use units::{Box2DI32, HeightI32, PosXI32, PosYI32, Position2DI32, Size2DI32, UnitI32, WidthI32};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Mul, Div)]
pub struct ScreenChars(pub i32);

impl UnitI32 for ScreenChars {
    fn new(value: i32) -> Self {
        Self(value)
    }

    fn zero() -> Self {
        Self(0 as i32)
    }

    fn to_primitive(&self) -> i32 {
        self.0
    }

    fn abs(&self) -> Self {
        Self(self.0.abs())
    }
}

impl ScreenChars {
    pub fn new_width(value: i32) -> WidthI32<Self> {
        WidthI32(Self(value))
    }

    pub fn new_height(value: i32) -> HeightI32<Self> {
        HeightI32(Self(value))
    }

    pub fn new_size2d(width: i32, height: i32) -> Size2DI32<Self> {
        Size2DI32 {
            width: Self::new_width(width),
            height: Self::new_height(height),
        }
    }

    pub fn new_position2d(x: i32, y: i32) -> Position2DI32<Self> {
        Position2DI32 {
            x: PosXI32(Self(x)),
            y: PosYI32(Self(y)),
        }
    }

    pub fn new_position2d_from_point(point: Point) -> Position2DI32<Self> {
        Position2DI32::<Self>::from_bracket_geometry_point(point)
    }

    pub fn new_box2d(p1: Position2DI32<Self>, p2: Position2DI32<Self>) -> Box2DI32<Self> {
        Box2DI32 { p1: p1, p2: p2 }
    }

    pub fn new_box2d_from_x1_y1_x2_y2(x1: i32, y1: i32, x2: i32, y2: i32) -> Box2DI32<Self> {
        Self::new_box2d(Self::new_position2d(x1, y1), Self::new_position2d(x2, y2))
    }

    pub fn new_box2d_from_position_and_size(
        position: Position2DI32<Self>,
        size: Size2DI32<Self>,
    ) -> Box2DI32<Self> {
        Box2DI32::new_from_position_and_size(position, size)
    }

    pub fn new_box2d_from_size(size: Size2DI32<Self>) -> Box2DI32<Self> {
        Box2DI32::new_from_size(size)
    }

    pub fn new_box2d_from_width_height(width: i32, height: i32) -> Box2DI32<Self> {
        Self::new_box2d_from_size(Self::new_size2d(width, height))
    }
}
