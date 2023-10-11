use crate::ui::units::{Box2D, Height, Pixels, Point2D, PosX, PosY, Size2D, Width};

#[test]
fn pixels_add() {
    assert_eq!(Pixels(1) + Pixels(2), Pixels(3));
}

#[test]
fn pixels_sub() {
    assert_eq!(Pixels(3) - Pixels(2), Pixels(1));
}

#[test]
fn pixels_mul() {
    assert_eq!(Pixels(3) * 4, Pixels(12));
}

#[test]
fn pixels_new_width_height_posx_posy() {
    assert_eq!(Pixels::new_width(3), Width(Pixels(3)));
    assert_eq!(Pixels::new_height(3), Height(Pixels(3)));
    assert_eq!(Pixels::new_posx(4), PosX(Pixels(4)));
    assert_eq!(Pixels::new_posy(5), PosY(Pixels(5)));
}

#[test]
fn pixels_new_point2d() {
    assert_eq!(
        Pixels::new_point2d(3, 4),
        Point2D::new_from_x_y(Pixels(3), Pixels(4))
    );
}

#[test]
fn pixels_new_size2d() {
    assert_eq!(
        Pixels::new_size2d(3, 4),
        Size2D::new_from_width_height(Pixels(3), Pixels(4))
    );
}

#[test]
fn pixels_new_box2d_from_x1_y1_x2_y2() {
    assert_eq!(
        Pixels::new_box2d_from_x1_y1_x2_y2(1, 2, 3, 5),
        Box2D::new_from_p1_p2(Pixels::new_point2d(1, 2), Pixels::new_point2d(3, 5))
    );
}

#[test]
fn pixels_new_box2d_from_width_height() {
    assert_eq!(
        Pixels::new_box2d_from_width_height(3, 5),
        Box2D::new_from_p1_p2(Pixels::new_point2d(0, 0), Pixels::new_point2d(3, 5))
    );
}
