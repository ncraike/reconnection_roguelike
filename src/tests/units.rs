use crate::ui::units::{Pixels, Tiles1x, Width};

#[test]
fn tile1x_width_in_pixels() {
    let tiles_wide = Width::<Tiles1x>(Tiles1x(3));
    assert_eq!(Pixels::from(tiles_wide), Pixels(48));
}
