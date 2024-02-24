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
fn width_i32_mul_i32_gives_width_i32() {
    assert_eq!(WidthI32(MyUnitI32(4)) * 3, WidthI32(MyUnitI32(12)))
}

#[test]
fn width_i32_div_i32_gives_width_i32() {
    assert_eq!(WidthI32(MyUnitI32(8)) / 2, WidthI32(MyUnitI32(4)))
}

#[test]
fn height_i32_add() {
    assert_eq!(
        HeightI32(MyUnitI32(2)) + HeightI32(MyUnitI32(3)),
        HeightI32::<MyUnitI32>(MyUnitI32(5))
    );
}

#[test]
fn height_i32_mul_i32_gives_height_i32() {
    assert_eq!(HeightI32(MyUnitI32(3)) * 2, HeightI32(MyUnitI32(6)));
}

#[test]
fn height_i32_div_i32_gives_height_i32() {
    assert_eq!(HeightI32(MyUnitI32(12)) / 3, HeightI32(MyUnitI32(4)));
}
