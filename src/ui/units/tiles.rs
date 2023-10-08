extern crate derive_more;
use super::base::{Height, Pixels, Width};
use derive_more::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Add, Mul)]
pub struct Tiles1x(pub i32);

impl Tiles1x {
    pub fn new_width(value: i32) -> Width<Tiles1x> {
        Width::<Tiles1x>(Tiles1x(value))
    }

    pub fn new_height(value: i32) -> Height<Tiles1x> {
        Height::<Tiles1x>(Tiles1x(value))
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
    pub fn new_width(value: i32) -> Width<Tiles2x> {
        Width::<Tiles2x>(Tiles2x(value))
    }

    pub fn new_height(value: i32) -> Height<Tiles2x> {
        Height::<Tiles2x>(Tiles2x(value))
    }
}

impl From<Width<Tiles2x>> for Pixels {
    fn from(width: Width<Tiles2x>) -> Self {
        let quantity = width.0 .0;
        Pixels::from(Tiles1x::new_width(quantity * 2))
    }
}

impl From<Height<Tiles2x>> for Pixels {
    fn from(height: Height<Tiles2x>) -> Self {
        let quantity = height.0 .0;
        Pixels::from(Tiles1x::new_height(quantity * 2))
    }
}
