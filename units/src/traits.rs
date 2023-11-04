pub trait Unit {
    type ValueType;

    fn new(value: Self::ValueType) -> Self;
    fn zero() -> Self;
    fn to_primitive(&self) -> Self::ValueType;
    fn abs(&self) -> Self;
}
