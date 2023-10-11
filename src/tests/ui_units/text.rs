use crate::ui::units::{Pixels, TextChars};

#[test]
fn text_width_in_pixels() {
    let text_width = TextChars::new_width(4);
    assert_eq!(Pixels::from(text_width), Pixels(32));
}

#[test]
fn text_height_in_pixels() {
    let text_height = TextChars::new_height(3);
    assert_eq!(Pixels::from(text_height), Pixels(48));
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
