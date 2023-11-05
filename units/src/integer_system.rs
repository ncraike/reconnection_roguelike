pub enum XYAxes {
    X,
    Y,
}

pub trait IntegerUnit: Sized {
    type PrimitiveType: Sized + Clone;

    fn new(value: Self::PrimitiveType) -> Self;
    fn to_primitive(&self) -> Self::PrimitiveType;
    fn zero() -> Self;
    fn abs(&self) -> Self;
}

pub trait DerivedUnit: IntegerUnit {
    type BaseUnit: IntegerUnit;

    fn to_base_unit(&self) -> Self::BaseUnit;
    fn from_base_unit_to_floor(base_quantity: Self::BaseUnit) -> Self;
    fn from_base_unit_to_ceil(base_quantity: Self::BaseUnit) -> Self;
}

pub trait ConvertibleUnit: DerivedUnit {
    type OtherUnit: DerivedUnit<BaseUnit = Self::BaseUnit>;

    fn convert_to_floor(&self) -> Self::OtherUnit;
    fn convert_to_ceil(&self) -> Self::OtherUnit;
}

pub trait DerivedUnitDisparateXY: IntegerUnit {
    type BaseUnit: IntegerUnit;

    fn to_base_unit(&self, in_axis: XYAxes) -> Self::BaseUnit;
    fn from_base_unit_to_floor(base_quantity: Self::BaseUnit, in_axis: XYAxes) -> Self;
    fn from_base_unit_to_ceil(base_quantity: Self::BaseUnit, in_axis: XYAxes) -> Self;
}

pub trait ConvertibleUnitDisparateXY: DerivedUnit {
    type OtherUnit: DerivedUnit<BaseUnit = Self::BaseUnit>;

    fn convert_to_floor(&self, in_axis: XYAxes) -> Self::OtherUnit;
    fn convert_to_ceil(&self, in_axis: XYAxes) -> Self::OtherUnit;
}
