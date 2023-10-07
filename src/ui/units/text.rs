use super::base::{Height, Pixels, Width};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Text(pub i32);

pub type TextWidth = Width<Text>;
pub type TextHeight = Height<Text>;

impl From<TextWidth> for Pixels {
    fn from(width: TextWidth) -> Self {
        let quantity = width.0 .0;
        Pixels(quantity * 8)
    }
}

impl From<TextHeight> for Pixels {
    fn from(height: TextHeight) -> Self {
        let quantity = height.0 .0;
        Pixels(quantity * 16)
    }
}
