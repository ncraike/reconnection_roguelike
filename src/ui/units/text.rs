extern crate derive_more;
use super::base::{Height, Pixels, Width};
use super::traits::Unit;
use derive_more::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Mul, Sub)]
pub struct TextChars(pub i32);

impl TextChars {
    pub fn new_width(value: i32) -> Width<TextChars> {
        Width::<TextChars>(TextChars(value))
    }

    pub fn new_height(value: i32) -> Height<TextChars> {
        Height::<TextChars>(TextChars(value))
    }
}

impl Unit for TextChars {
    type ValueType = i32;

    fn new(value: i32) -> TextChars {
        TextChars(value)
    }

    fn zero() -> TextChars {
        TextChars(0)
    }

    fn value(&self) -> i32 {
        self.0
    }
}

impl From<Width<TextChars>> for Pixels {
    fn from(width: Width<TextChars>) -> Self {
        let quantity = width.0 .0;
        Pixels(quantity * 8)
    }
}

impl From<Height<TextChars>> for Pixels {
    fn from(height: Height<TextChars>) -> Self {
        let quantity = height.0 .0;
        Pixels(quantity * 16)
    }
}
