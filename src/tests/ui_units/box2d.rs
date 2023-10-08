use crate::ui::units::{Box2D, Height, Pixels, Point2D, PosX, PosY, Width};

#[test]
fn new_pixels_box2d() {
    let top_left = Point2D {
        x: PosX(Pixels(1)),
        y: PosY(Pixels(1)),
    };
    let bot_right = Point2D {
        x: PosX(Pixels(2)),
        y: PosY(Pixels(3)),
    };
    let box2d = Box2D {
        p1: top_left,
        p2: bot_right,
    };
    assert_eq!(box2d.x1(), PosX(Pixels(1)));
    assert_eq!(box2d.y2(), PosY(Pixels(3)));
    assert_eq!(box2d.width(), Width(Pixels(1)));
    assert_eq!(box2d.height(), Height(Pixels(2)));
}
