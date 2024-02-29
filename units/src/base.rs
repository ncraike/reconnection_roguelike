pub trait Unit {
    fn new(value: i32) -> Self;
    fn zero() -> Self;
    fn to_primitive(&self) -> i32;
    fn abs(&self) -> Self;
}
