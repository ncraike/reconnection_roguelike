use super::base::{Height, Pixels, Width};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Text(pub i32);

pub type TextWidth = Width<Text>;
pub type TextHeight = Height<Text>;

impl Text {
    pub fn new_width(value: i32) -> Width<Text> {
        Width::<Text>(Text(value))
    }

    pub fn new_height(value: i32) -> Height<Text> {
        Height::<Text>(Text(value))
    }
}

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
