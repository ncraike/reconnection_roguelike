use crate::ui::units::{Height, Pixels, TextChars, Tiles1x, Tiles2x, Width};

#[test]
fn text_width_in_pixels() {
    let text_width = TextChars::new_width(4);
    assert_eq!(Width::<Pixels>::from(text_width), Pixels::new_width(32));
}

#[test]
fn text_height_in_pixels() {
    let text_height = TextChars::new_height(3);
    assert_eq!(Height::<Pixels>::from(text_height), Pixels::new_height(48));
}

#[test]
fn text_chars_height_to_tiles1x_floor() {
    assert_eq!(
        TextChars::new_height(4).to_tiles1x_floor(),
        Tiles1x::new_height(2)
    );
}

#[test]
fn text_width_add() {
    assert_eq!(
        TextChars::new_width(1) + TextChars::new_width(2),
        TextChars::new_width(3)
    );
}

#[test]
fn text_width_mul() {
    assert_eq!(TextChars::new_width(3) * 4, TextChars::new_width(12));
}
