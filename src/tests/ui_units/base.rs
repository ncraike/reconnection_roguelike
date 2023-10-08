use crate::ui::units::{Pixels, Width};

#[test]
fn pixels_add() {
    assert_eq!(Pixels(1) + Pixels(2), Pixels(3));
}

#[test]
fn pixels_mul() {
    assert_eq!(Pixels(3) * 4, Pixels(12));
}

#[test]
fn width_add() {
    assert_eq!(
        Width::<Pixels>(Pixels(1)) + Width::<Pixels>(Pixels(2)),
        Width::<Pixels>(Pixels(3))
    );
}
