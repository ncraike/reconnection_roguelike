use super::base::MyUnitI32;
use crate::{HeightI32, WidthI32};

#[test]
fn width_i32_add() {
    assert_eq!(
        WidthI32(MyUnitI32(1)) + WidthI32(MyUnitI32(2)),
        WidthI32::<MyUnitI32>(MyUnitI32(3))
    );
}

#[test]
fn height_i32_add() {
    assert_eq!(
        HeightI32(MyUnitI32(2)) + HeightI32(MyUnitI32(3)),
        HeightI32::<MyUnitI32>(MyUnitI32(5))
    );
}
