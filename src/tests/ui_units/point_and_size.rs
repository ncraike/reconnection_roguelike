use crate::ui::units::{Height, Pixels, Point2D, PosX, PosY, Size2D, Width};

#[test]
fn point2d_add_size2d() {
    let point2d = Point2D {
        x: PosX(Pixels(3)),
        y: PosY(Pixels(4)),
    };
    let size2d = Size2D {
        w: Width(Pixels(2)),
        h: Height(Pixels(3)),
    };
    let expected = Point2D {
        x: PosX(Pixels(5)),
        y: PosY(Pixels(7)),
    };
    assert_eq!(point2d + size2d, expected);
}

#[test]
fn size2d_add_size2d() {
    assert_eq!(
        Size2D {
            w: Width(Pixels(1)),
            h: Height(Pixels(2)),
        } + Size2D {
            w: Width(Pixels(3)),
            h: Height(Pixels(4)),
        },
        Size2D {
            w: Width(Pixels(4)),
            h: Height(Pixels(6)),
        }
    );
}
