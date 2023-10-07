use super::base::{Height, Pixels, Width};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tiles1x(pub i32);

pub type Tiles1xWidth = Width<Tiles1x>;
pub type Tiles1xHeight = Height<Tiles1x>;

impl From<Tiles1xWidth> for Pixels {
    fn from(width: Tiles1xWidth) -> Self {
        let quantity = width.0 .0;
        Pixels(quantity * 16)
    }
}

impl From<Tiles1xHeight> for Pixels {
    fn from(height: Tiles1xHeight) -> Self {
        let quantity = height.0 .0;
        Pixels(quantity * 24)
    }
}
