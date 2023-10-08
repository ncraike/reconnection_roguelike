use crate::ui::units::{Pixels, Tiles1x, Tiles2x};

#[test]
fn tiles1x_width_in_pixels() {
    let tiles_wide = Tiles1x::new_width(3);
    assert_eq!(Pixels::from(tiles_wide), Pixels(48));
}

#[test]
fn tiles1x_height_in_pixels() {
    let tiles_high = Tiles1x::new_height(5);
    assert_eq!(Pixels::from(tiles_high), Pixels(120));
}

#[test]
fn tiles2x_width_in_pixels() {
    let tiles2x_wide = Tiles2x::new_width(3);
    assert_eq!(Pixels::from(tiles2x_wide), Pixels(96));
}

#[test]
fn tiles2x_height_in_pixels() {
    let tiles2x_high = Tiles2x::new_height(5);
    assert_eq!(Pixels::from(tiles2x_high), Pixels(240));
}
