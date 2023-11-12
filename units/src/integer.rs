pub enum XYAxes {
    X,
    Y,
}

pub trait IntegerUnit<PrimitiveType>: Sized
where
    PrimitiveType: Sized + Clone,
{
    fn new(quantity: PrimitiveType) -> Self;
    fn to_primitive(&self) -> PrimitiveType;
    fn zero() -> Self;
    fn abs(&self) -> Self;
}

pub trait DerivedIntegerUnit<PrimitiveType, BaseUnit>: IntegerUnit<PrimitiveType>
where
    BaseUnit: IntegerUnit<PrimitiveType>,
    PrimitiveType: Sized + Clone,
{
    fn to_base_unit(&self) -> BaseUnit;
    fn from_base_unit_to_floor(base_quantity: BaseUnit) -> Self;
    fn from_base_unit_to_ceil(base_quantity: BaseUnit) -> Self;
}

pub trait ConvertibleIntegerUnit<PrimitiveType, BaseUnit, OtherUnit>:
    DerivedIntegerUnit<PrimitiveType, BaseUnit>
where
    OtherUnit: DerivedIntegerUnit<PrimitiveType, BaseUnit>,
    BaseUnit: IntegerUnit<PrimitiveType>,
    PrimitiveType: Sized + Clone,
{
    fn convert_to_floor(&self) -> OtherUnit;
    fn convert_to_ceil(&self) -> OtherUnit;
}

pub trait DerivedIntegerUnitDisparateXY<PrimitiveType, BaseUnit>:
    IntegerUnit<PrimitiveType>
where
    BaseUnit: IntegerUnit<PrimitiveType>,
    PrimitiveType: Sized + Clone,
{
    fn to_base_unit(&self, in_axis: XYAxes) -> BaseUnit;
    fn from_base_unit_to_floor(base_quantity: BaseUnit, in_axis: XYAxes) -> Self;
    fn from_base_unit_to_ceil(base_quantity: BaseUnit, in_axis: XYAxes) -> Self;
}

pub trait ConvertibleIntegerUnitDisparateXY<PrimitiveType, BaseUnit, OtherUnit>:
    DerivedIntegerUnitDisparateXY<PrimitiveType, BaseUnit>
where
    BaseUnit: IntegerUnit<PrimitiveType>,
    OtherUnit: DerivedIntegerUnitDisparateXY<PrimitiveType, BaseUnit>,
    PrimitiveType: Sized + Clone,
{
    fn convert_to_floor(&self, in_axis: XYAxes) -> OtherUnit;
    fn convert_to_ceil(&self, in_axis: XYAxes) -> OtherUnit;
}
