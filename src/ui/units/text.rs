extern crate derive_more;
use super::base::{Height, Pixels, Width};
use super::traits::Unit;
use derive_more::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Add, Mul)]
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

impl Unit for Text {
    type ValueType = i32;

    fn new(value: i32) -> Text {
        Text(value)
    }

    fn zero() -> Text {
        Text(0)
    }

    fn value(&self) -> i32 {
        self.0
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
