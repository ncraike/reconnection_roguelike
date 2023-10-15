use crate::ui::units::{
    Height, Pixels, Point2D, PosX, PosY, Size2D, TextChars, Tiles1x, Tiles2x, Width,
};

#[test]
fn tiles1x_add() {
    assert_eq!(Tiles1x(1) + Tiles1x(2), Tiles1x(3));
}

#[test]
fn tiles1x_mul() {
    assert_eq!(Tiles1x(3) * 4, Tiles1x(12));
}

#[test]
fn tiles1x_width_in_pixels() {
    let tiles_wide = Width(Tiles1x(1));
    assert_eq!(tiles_wide.to_pixels(), Pixels::new_width(16));
}

#[test]
fn tiles1x_3_width_in_pixels() {
    let tiles_wide = Width(Tiles1x(3));
    assert_eq!(tiles_wide.to_pixels(), Pixels::new_width(48));
}

#[test]
fn tiles1x_3_width_to_text_chars_floor() {
    assert_eq!(
        Tiles1x::new_width(3).to_text_chars_floor(),
        TextChars::new_width(6)
    );
}

#[test]
fn tiles1x_width_add() {
    assert_eq!(Width(Tiles1x(1)) + Width(Tiles1x(2)), Width(Tiles1x(3)));
}

#[test]
fn tiles1x_width_mul() {
    assert_eq!(Width(Tiles1x(3)) * 4, Width(Tiles1x(12)));
}

#[test]
fn tiles1x_height_in_pixels() {
    let tiles_high = Height(Tiles1x(1));
    assert_eq!(tiles_high.to_pixels(), Pixels::new_height(24));
}

#[test]
fn tiles1x_5_height_in_pixels() {
    let tiles_high = Height(Tiles1x(5));
    assert_eq!(tiles_high.to_pixels(), Pixels::new_height(120));
}

#[test]
fn tiles1x_height_from_pixels_floor() {
    assert_eq!(
        Height::<Tiles1x>::from_pixels_floor(Pixels::new_height(51)),
        Tiles1x::new_height(2)
    );
}

#[test]
fn tiles1x_height_from_pixels_ceil() {
    assert_eq!(
        Height::<Tiles1x>::from_pixels_ceil(Pixels::new_height(51)),
        Tiles1x::new_height(3)
    );
}

#[test]
fn tiles1x_new_point2d() {
    assert_eq!(
        Tiles1x::new_point2d(3, 4),
        Point2D {
            x: PosX(Tiles1x(3)),
            y: PosY(Tiles1x(4)),
        }
    );
}

#[test]
fn tiles1x_new_size2d() {
    assert_eq!(
        Tiles1x::new_size2d(1, 2),
        Size2D {
            width: Width(Tiles1x(1)),
            height: Height(Tiles1x(2)),
        }
    );
}

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
