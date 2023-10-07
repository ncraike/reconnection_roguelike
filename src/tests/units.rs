use crate::ui::units::{Pixels, Tiles1x, Width};

#[test]
fn tile1x_width_in_pixels() {
    let tiles_wide = Tiles1x::new_width(3);
    assert_eq!(Pixels::from(tiles_wide), Pixels(48));
}

#[test]
fn tile1x_height_in_pixels() {
    let tiles_high = Tiles1x::new_height(5);
    assert_eq!(Pixels::from(tiles_high), Pixels(120));
}
