use crate::{Box2D, Height, Pixels, PosX, PosY, Width};

#[test]
fn box2d_new_from_p1_p2() {
    let box2d =
        Box2D::<Pixels>::new_from_p1_p2(Pixels::new_point2d(1, 2), Pixels::new_point2d(3, 5));
    assert_eq!(box2d.x1(), PosX(Pixels(1)));
    assert_eq!(box2d.y1(), PosY(Pixels(2)));
    assert_eq!(box2d.x2(), PosX(Pixels(3)));
    assert_eq!(box2d.y2(), PosY(Pixels(5)));
}

#[test]
fn box2d_new_from_point_and_size() {
    let box2d = Box2D::new_from_point_and_size(Pixels::new_point2d(1, 2), Pixels::new_size2d(3, 4));
    assert_eq!(box2d.x1(), PosX(Pixels(1)));
    assert_eq!(box2d.y1(), PosY(Pixels(2)));
    assert_eq!(box2d.x2(), PosX(Pixels(4)));
    assert_eq!(box2d.y2(), PosY(Pixels(6)));
}

#[test]
fn box2d_new_from_size() {
    let box2d = Box2D::new_from_size(Pixels::new_size2d(3, 4));
    assert_eq!(box2d.x1(), PosX(Pixels(0)));
    assert_eq!(box2d.y1(), PosY(Pixels(0)));
    assert_eq!(box2d.x2(), PosX(Pixels(3)));
    assert_eq!(box2d.y2(), PosY(Pixels(4)));
}

#[test]
fn box2d_new_from_width_and_height() {
    let box2d = Box2D::new_from_width_height(Width(Pixels(3)), Height(Pixels(4)));
    assert_eq!(box2d.x1(), PosX(Pixels(0)));
    assert_eq!(box2d.y1(), PosY(Pixels(0)));
    assert_eq!(box2d.x2(), PosX(Pixels(3)));
    assert_eq!(box2d.y2(), PosY(Pixels(4)));
}
