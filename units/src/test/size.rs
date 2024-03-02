use crate::example::MyUnit;
use crate::Height;

#[test]
fn height_i32_add() {
    assert_eq!(
        Height(MyUnit(2)) + Height(MyUnit(3)),
        Height::<MyUnit>(MyUnit(5))
    );
}

#[test]
fn height_i32_mul_i32_gives_height_i32() {
    assert_eq!(Height(MyUnit(3)) * 2, Height(MyUnit(6)));
}

#[test]
fn height_i32_div_i32_gives_height_i32() {
    assert_eq!(Height(MyUnit(12)) / 3, Height(MyUnit(4)));
}
