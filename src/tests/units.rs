use crate::ui::units::{Pixels, Text, Tiles1x, Tiles2x, Width};

#[test]
fn pixel_add() {
    assert_eq!(Pixels(1) + Pixels(2), Pixels(3));
}

#[test]
fn pixel_mul() {
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
fn tile1x_width_in_pixels() {
    let tiles_wide = Tiles1x::new_width(3);
    assert_eq!(Pixels::from(tiles_wide), Pixels(48));
}

#[test]
fn tile1x_height_in_pixels() {
    let tiles_high = Tiles1x::new_height(5);
    assert_eq!(Pixels::from(tiles_high), Pixels(120));
}

#[test]
fn tile2x_width_in_pixels() {
    let tiles2x_wide = Tiles2x::new_width(3);
    assert_eq!(Pixels::from(tiles2x_wide), Pixels(96));
}

#[test]
fn tile2x_height_in_pixels() {
    let tiles2x_high = Tiles2x::new_height(5);
    assert_eq!(Pixels::from(tiles2x_high), Pixels(240));
}

#[test]
fn text_width_in_pixels() {
    let text_width = Text::new_width(4);
    assert_eq!(Pixels::from(text_width), Pixels(32));
}

#[test]
fn text_height_in_pixels() {
    let text_height = Text::new_height(3);
    assert_eq!(Pixels::from(text_height), Pixels(48));
}
