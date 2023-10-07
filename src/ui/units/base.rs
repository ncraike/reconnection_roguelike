#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixels(pub i32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Width<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Height<T>(pub T);
