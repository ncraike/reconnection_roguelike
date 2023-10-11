pub trait Unit {
    type ValueType;

    fn new(value: Self::ValueType) -> Self;
    fn zero() -> Self;
    fn value(&self) -> Self::ValueType;
    fn abs(&self) -> Self::ValueType;
}
