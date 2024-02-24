use super::base::MyUnitI32;
use crate::{HeightI32, PosXI32, PosYI32, WidthI32};

#[test]
fn pos_x_i32_add() {
    let x = PosXI32(MyUnitI32(3));
    let w = WidthI32(MyUnitI32(4));
    assert_eq!(x + w, PosXI32::<MyUnitI32>(MyUnitI32(7)));
}

#[test]
fn pos_x_i32_sub() {
    let x1 = PosXI32(MyUnitI32(3));
    let x2 = PosXI32(MyUnitI32(7));
    assert_eq!(x2 - x1, WidthI32::<MyUnitI32>(MyUnitI32(4)));
}

#[test]
fn pos_y_i32_add() {
    let y = PosYI32(MyUnitI32(2));
    let h = HeightI32(MyUnitI32(3));
    assert_eq!(y + h, PosYI32::<MyUnitI32>(MyUnitI32(5)));
}

#[test]
fn pos_y_i32_sub() {
    let y1 = PosYI32(MyUnitI32(2));
    let y2 = PosYI32(MyUnitI32(5));
    assert_eq!(y2 - y1, HeightI32::<MyUnitI32>(MyUnitI32(3)));
}
