use crate::ui::units::{Height, Pixels, Point2D, PosX, PosY, Size2D, Width};

#[test]
fn point2d_new_from_x_y() {
    let point2d = Point2D::new_from_x_y(PosX(Pixels(3)), PosY(Pixels(4)));
    assert_eq!(point2d.x, PosX(Pixels(3)));
    assert_eq!(point2d.y, PosY(Pixels(4)));
}

#[test]
fn point2d_origin() {
    let origin = Point2D::<Pixels>::origin();
    assert_eq!(origin.x, PosX(Pixels(0)));
    assert_eq!(origin.y, PosY(Pixels(0)));
}

#[test]
fn point2d_new_box2d_from_other_point() {
    let p1 = Pixels::new_point2d(1, 2);
    let p2 = Pixels::new_point2d(3, 5);
    let box2d = p1.new_box2d_from_other_point(p2);
    assert_eq!(box2d, Pixels::new_box2d_from_x1_y1_x2_y2(1, 2, 3, 5));
}

#[test]
fn point2d_new_box2d_from_size() {
    let p1 = Pixels::new_point2d(1, 2);
    let box2d = p1.new_box2d_from_size(Pixels::new_size2d(2, 3));
    assert_eq!(box2d, Pixels::new_box2d_from_x1_y1_x2_y2(1, 2, 3, 5));
}

#[test]
fn point2d_new_box2d_from_width_height() {
    let p1 = Pixels::new_point2d(1, 2);
    let box2d = p1.new_box2d_from_width_height(Pixels::new_width(2), Pixels::new_height(3));
    assert_eq!(box2d, Pixels::new_box2d_from_x1_y1_x2_y2(1, 2, 3, 5));
}

#[test]
fn size2d_new_from_width_height() {
    let size2d = Size2D::new_from_width_height(Pixels::new_width(3), Pixels::new_height(4));
    assert_eq!(size2d.w, Width(Pixels(3)));
    assert_eq!(size2d.h, Height(Pixels(4)));
}

#[test]
fn size2d_nothing() {
    let size2d = Size2D::<Pixels>::nothing();
    assert_eq!(size2d.w, Width(Pixels(0)));
    assert_eq!(size2d.h, Height(Pixels(0)));
}

#[test]
fn size2d_abs() {
    assert_eq!(Pixels::new_size2d(-3, -4).abs(), Pixels::new_size2d(3, 4),);
}

#[test]
fn point2d_add_size2d() {
    assert_eq!(
        Pixels::new_point2d(3, 4) + Pixels::new_size2d(2, 3),
        Pixels::new_point2d(5, 7)
    );
}

#[test]
fn point2d_sub_point2d_gives_size2d() {
    assert_eq!(
        Pixels::new_point2d(5, 7) - Pixels::new_point2d(3, 4),
        Pixels::new_size2d(2, 3)
    );
}
