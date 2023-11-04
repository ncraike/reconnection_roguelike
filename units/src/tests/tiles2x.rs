use crate::{Height, Pixels, Point2D, PosX, PosY, Size2D, TextChars, Tiles1x, Tiles2x, Width};

#[test]
fn tiles2x_width_in_pixels() {
    let tiles2x_wide = Width(Tiles2x(3));
    assert_eq!(tiles2x_wide.to_pixels(), Pixels::new_width(96));
}

#[test]
fn tiles2x_height_in_pixels() {
    let tiles2x_high = Height(Tiles2x(5));
    assert_eq!(tiles2x_high.to_pixels(), Pixels::new_height(240));
}

#[test]
fn tiles2x_height_add() {
    assert_eq!(Height(Tiles2x(1)) + Height(Tiles2x(2)), Height(Tiles2x(3)));
}

#[test]
fn tiles2x_new_point2d() {
    assert_eq!(
        Tiles2x::new_point2d(3, 4),
        Point2D {
            x: PosX(Tiles2x(3)),
            y: PosY(Tiles2x(4)),
        }
    );
}

#[test]
fn tiles2x_new_size2d() {
    assert_eq!(
        Tiles2x::new_size2d(1, 2),
        Size2D {
            width: Width(Tiles2x(1)),
            height: Height(Tiles2x(2)),
        }
    );
}

#[test]
fn tiles2x_size_to_pixels() {
    assert_eq!(
        Tiles2x::new_size2d(4, 3).to_pixels(),
        Pixels::new_size2d(128, 144)
    );
}

#[test]
fn tiles2x_size_from_pixels_floor() {
    assert_eq!(
        Size2D::<Tiles2x>::from_pixels_floor(Pixels::new_size2d(130, 145)),
        Tiles2x::new_size2d(4, 3)
    )
}

#[test]
fn tiles2x_size_from_pixels_ceil() {
    assert_eq!(
        Size2D::<Tiles2x>::from_pixels_ceil(Pixels::new_size2d(127, 141)),
        Tiles2x::new_size2d(4, 3)
    )
}

#[test]
fn tiles2x_size_to_text_chars_floor() {
    assert_eq!(
        Tiles2x::new_size2d(5, 3).to_text_chars_floor(),
        TextChars::new_size2d(20, 9)
    )
}

#[test]
fn tiles2x_size_to_text_chars_ceil() {
    assert_eq!(
        Tiles2x::new_size2d(4, 2).to_text_chars_ceil(),
        TextChars::new_size2d(16, 6)
    )
}

#[test]
fn tiles2x_size_to_tiles1x() {
    assert_eq!(
        Tiles2x::new_size2d(6, 4).to_tiles1x(),
        Tiles1x::new_size2d(12, 8)
    )
}
