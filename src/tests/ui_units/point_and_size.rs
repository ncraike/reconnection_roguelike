use crate::ui::units::{Height, Pixels, Point2D, PosX, PosY, Size2D, Width};

#[test]
fn point2d_new_from_x_y() {
    let point2d = Point2D::new_from_x_y(Pixels(3), Pixels(4));
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
fn size2d_new() {
    let size2d = Size2D::new_from_width_height(Pixels(3), Pixels(4));
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
    assert_eq!(
        Size2D::new_from_width_height(Pixels(-3), Pixels(-4)).abs(),
        Size2D::new_from_width_height(Pixels(3), Pixels(4)),
    );
}

#[test]
fn point2d_add_size2d() {
    assert_eq!(
        Point2D::new_from_x_y(Pixels(3), Pixels(4))
            + Size2D::new_from_width_height(Pixels(2), Pixels(3)),
        Point2D::new_from_x_y(Pixels(5), Pixels(7))
    );
}

#[test]
fn point2d_sub_point2d_gives_size2d() {
    assert_eq!(
        Point2D::new_from_x_y(Pixels(5), Pixels(7)) - Point2D::new_from_x_y(Pixels(3), Pixels(4)),
        Size2D::new_from_width_height(Pixels(2), Pixels(3))
    );
}
