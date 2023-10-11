use crate::ui::units::{Height, Pixels, Width};

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

#[test]
fn width_add() {
    assert_eq!(
        Width::<Pixels>(Pixels(1)) + Width::<Pixels>(Pixels(2)),
        Width::<Pixels>(Pixels(3))
    );
}

#[test]
fn width_mul() {
    assert_eq!(Width::<Pixels>(Pixels(3)) * 4, Width::<Pixels>(Pixels(12)));
}

#[test]
fn height_add() {
    assert_eq!(
        Height::<Pixels>(Pixels(1)) + Height::<Pixels>(Pixels(2)),
        Height::<Pixels>(Pixels(3))
    );
}

#[test]
fn height_mul() {
    assert_eq!(
        Height::<Pixels>(Pixels(3)) * 4,
        Height::<Pixels>(Pixels(12))
    );
}
