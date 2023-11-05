pub trait IntegerUnitI32: Sized + From<i32> {
    fn new(value: i32) -> Self;
    fn to_primitive(&self) -> i32;

    fn zero() -> Self {
        Self::new(0 as i32)
    }

    fn abs(&self) -> Self {
        Self::new(self.to_primitive().abs())
    }
}

pub trait BaseUnitI32: IntegerUnitI32 {}

pub trait DerivedUnitI32: IntegerUnitI32 {
    type BaseUnit: BaseUnitI32;
}
