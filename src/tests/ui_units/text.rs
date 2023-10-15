use crate::ui::units::{Height, Pixels, Size2D, TextChars, Tiles1x, Width};

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

#[test]
fn text_size_to_pixels() {
    assert_eq!(
        TextChars::new_size2d(4, 3).to_pixels(),
        Pixels::new_size2d(32, 48)
    );
}

#[test]
fn text_size_from_pixels_floor() {
    assert_eq!(
        Size2D::<TextChars>::from_pixels_floor(Pixels::new_size2d(34, 51)),
        TextChars::new_size2d(4, 3)
    )
}

#[test]
fn text_size_from_pixels_ceil() {
    assert_eq!(
        Size2D::<TextChars>::from_pixels_ceil(Pixels::new_size2d(30, 47)),
        TextChars::new_size2d(4, 3)
    )
}
