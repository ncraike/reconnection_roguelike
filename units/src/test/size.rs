use super::base::MyUnitI32;
use crate::{Height, Width};

#[test]
fn width_i32_add() {
    assert_eq!(
        Width(MyUnitI32(1)) + Width(MyUnitI32(2)),
        Width::<MyUnitI32>(MyUnitI32(3))
    );
}

#[test]
fn width_i32_mul_i32_gives_width_i32() {
    assert_eq!(Width(MyUnitI32(4)) * 3, Width(MyUnitI32(12)))
}

#[test]
fn width_i32_div_i32_gives_width_i32() {
    assert_eq!(Width(MyUnitI32(8)) / 2, Width(MyUnitI32(4)))
}

#[test]
fn height_i32_add() {
    assert_eq!(
        Height(MyUnitI32(2)) + Height(MyUnitI32(3)),
        Height::<MyUnitI32>(MyUnitI32(5))
    );
}

#[test]
fn height_i32_mul_i32_gives_height_i32() {
    assert_eq!(Height(MyUnitI32(3)) * 2, Height(MyUnitI32(6)));
}

#[test]
fn height_i32_div_i32_gives_height_i32() {
    assert_eq!(Height(MyUnitI32(12)) / 3, Height(MyUnitI32(4)));
}
