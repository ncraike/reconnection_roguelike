use crate::ui::units::Pixels;

#[test]
fn pixels_add() {
    assert_eq!(Pixels(1) + Pixels(2), Pixels(3));
}

#[test]
fn pixels_sub() {
    assert_eq!(Pixels(3) - Pixels(2), Pixels(1));
}

#[test]
fn pixels_mul() {
    assert_eq!(Pixels(3) * 4, Pixels(12));
}
