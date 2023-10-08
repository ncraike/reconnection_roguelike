extern crate derive_more;
use derive_more::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Add, Mul)]
pub struct Pixels(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Add, Mul)]
pub struct Width<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Add, Mul)]
pub struct Height<T>(pub T);
