extern crate derive_more;
use super::base::{Height, Pixels, Width};
use super::point_and_size::{Point2D, Size2D};
use super::pos_x_y::{PosX, PosY};
use derive_more::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Add, Mul)]
pub struct Tiles1x(pub i32);

impl Tiles1x {
    pub fn new_point2d(x: i32, y: i32) -> Point2D<Tiles1x> {
        Point2D::<Tiles1x> {
            x: PosX(Tiles1x(x)),
            y: PosY(Tiles1x(y)),
        }
    }

    pub fn new_size2d(w: i32, h: i32) -> Size2D<Tiles1x> {
        Size2D::<Tiles1x> {
            w: Width(Tiles1x(w)),
            h: Height(Tiles1x(h)),
        }
    }
}

impl From<Width<Tiles1x>> for Pixels {
    fn from(width: Width<Tiles1x>) -> Self {
        let quantity = width.0 .0;
        Pixels(quantity * 16)
    }
}

impl From<Height<Tiles1x>> for Pixels {
    fn from(height: Height<Tiles1x>) -> Self {
        let quantity = height.0 .0;
        Pixels(quantity * 24)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Add, Mul)]
pub struct Tiles2x(pub i32);

impl Tiles2x {
    pub fn new_point2d(x: i32, y: i32) -> Point2D<Tiles2x> {
        Point2D::<Tiles2x> {
            x: PosX(Tiles2x(x)),
            y: PosY(Tiles2x(y)),
        }
    }

    pub fn new_size2d(w: i32, h: i32) -> Size2D<Tiles2x> {
        Size2D::<Tiles2x> {
            w: Width(Tiles2x(w)),
            h: Height(Tiles2x(h)),
        }
    }
}

impl From<Width<Tiles2x>> for Pixels {
    fn from(width: Width<Tiles2x>) -> Self {
        let quantity = width.0 .0;
        Pixels::from(Width(Tiles1x(quantity * 2)))
    }
}

impl From<Height<Tiles2x>> for Pixels {
    fn from(height: Height<Tiles2x>) -> Self {
        let quantity = height.0 .0;
        Pixels::from(Height(Tiles1x(quantity * 2)))
    }
}
