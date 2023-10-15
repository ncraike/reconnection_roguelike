use crate::ui::units::{Height, Pixels, PosX, PosY, Width};
use std::cmp::min;

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
fn width_abs() {
    assert_eq!(Width(Pixels(-3)).abs(), Width(Pixels(3)));
}

#[test]
fn width_to_primitive() {
    assert_eq!(Width(Pixels(3)).to_primitive(), 3)
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

#[test]
fn height_to_primitive() {
    assert_eq!(Height(Pixels(5)).to_primitive(), 5)
}

#[test]
fn posx_to_primitive() {
    assert_eq!(PosX(Pixels(4)).to_primitive(), 4)
}

#[test]
fn posx_add_width() {
    assert_eq!(PosX(Pixels(4)) + Width(Pixels(3)), PosX(Pixels(7)))
}

#[test]
fn posx_ord() {
    assert!(PosX(Pixels(7)) > PosX(Pixels(5)));
}

#[test]
fn posx_min() {
    assert_eq!(min(PosX(Pixels(3)), PosX(Pixels(5))), PosX(Pixels(3)));
}

#[test]
fn posx_sub_posx_gives_width() {
    assert_eq!(PosX(Pixels(7)) - PosX(Pixels(3)), Width(Pixels(4)))
}

#[test]
fn posy_to_primitive() {
    assert_eq!(PosY(Pixels(2)).to_primitive(), 2)
}

#[test]
fn posy_add_height() {
    assert_eq!(PosY(Pixels(4)) + Height(Pixels(3)), PosY(Pixels(7)))
}

#[test]
fn posy_gt() {
    assert!(PosY(Pixels(5)) > PosY(Pixels(3)));
}

#[test]
fn posy_min() {
    assert_eq!(min(PosY(Pixels(3)), PosY(Pixels(5))), PosY(Pixels(3)));
}

#[test]
fn posy_sub_posy_gives_height() {
    assert_eq!(PosY(Pixels(7)) - PosY(Pixels(3)), Height(Pixels(4)))
}
