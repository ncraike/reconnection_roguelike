use crate::ui::units::{Height, Pixels, PosX, PosY, Width};

#[test]
fn posx_add_width() {
    assert_eq!(PosX(Pixels(4)) + Width(Pixels(3)), PosX(Pixels(7)))
}

#[test]
fn posy_add_height() {
    assert_eq!(PosY(Pixels(4)) + Height(Pixels(3)), PosY(Pixels(7)))
}
