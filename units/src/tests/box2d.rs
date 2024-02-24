use super::base::MyUnitI32;
use crate::{Box2DI32, HeightI32, PosXI32, PosYI32, Position2DI32, Size2DI32, WidthI32};

#[test]
fn x1_y1_x2_y2() {
    let box2d = Box2DI32::<MyUnitI32> {
        p1: Position2DI32 {
            x: PosXI32(MyUnitI32(2)),
            y: PosYI32(MyUnitI32(3)),
        },
        p2: Position2DI32 {
            x: PosXI32(MyUnitI32(4)),
            y: PosYI32(MyUnitI32(5)),
        },
    };
    assert_eq!(box2d.x1(), PosXI32(MyUnitI32(2)));
    assert_eq!(box2d.y1(), PosYI32(MyUnitI32(3)));
    assert_eq!(box2d.x2(), PosXI32(MyUnitI32(4)));
    assert_eq!(box2d.y2(), PosYI32(MyUnitI32(5)));
}

#[test]
fn size_width_height() {
    let box2d = Box2DI32::<MyUnitI32> {
        p1: Position2DI32 {
            x: PosXI32(MyUnitI32(2)),
            y: PosYI32(MyUnitI32(3)),
        },
        p2: Position2DI32 {
            x: PosXI32(MyUnitI32(4)),
            y: PosYI32(MyUnitI32(6)),
        },
    };
    assert_eq!(
        box2d.size(),
        Size2DI32 {
            width: WidthI32(MyUnitI32(2)),
            height: HeightI32(MyUnitI32(3)),
        }
    );
    assert_eq!(box2d.width(), WidthI32(MyUnitI32(2)));
    assert_eq!(box2d.height(), HeightI32(MyUnitI32(3)));
}

#[test]
fn normalize_does_not_change_already_normal() {
    let box2d = Box2DI32::<MyUnitI32> {
        p1: Position2DI32 {
            x: PosXI32(MyUnitI32(2)),
            y: PosYI32(MyUnitI32(3)),
        },
        p2: Position2DI32 {
            x: PosXI32(MyUnitI32(4)),
            y: PosYI32(MyUnitI32(5)),
        },
    };
    assert_eq!(box2d.normalize(), box2d);
}

#[test]
fn normalize_swaps_p1_p2() {
    let box2d = Box2DI32::<MyUnitI32> {
        p1: Position2DI32 {
            x: PosXI32(MyUnitI32(4)),
            y: PosYI32(MyUnitI32(5)),
        },
        p2: Position2DI32 {
            x: PosXI32(MyUnitI32(2)),
            y: PosYI32(MyUnitI32(3)),
        },
    };
    let normal_box2d = Box2DI32::<MyUnitI32> {
        p1: Position2DI32 {
            x: PosXI32(MyUnitI32(2)),
            y: PosYI32(MyUnitI32(3)),
        },
        p2: Position2DI32 {
            x: PosXI32(MyUnitI32(4)),
            y: PosYI32(MyUnitI32(5)),
        },
    };
    assert_eq!(box2d.normalize(), normal_box2d);
}

#[test]
fn normalize_picks_top_left_bottom_right_corners() {
    let box2d = Box2DI32::<MyUnitI32> {
        // Bottom-left corner (assuming top-left origin)
        p1: Position2DI32 {
            x: PosXI32(MyUnitI32(2)),
            y: PosYI32(MyUnitI32(5)),
        },
        // Top-right corner (assuming top-left origin)
        p2: Position2DI32 {
            x: PosXI32(MyUnitI32(4)),
            y: PosYI32(MyUnitI32(3)),
        },
    };
    let normal_box2d = Box2DI32::<MyUnitI32> {
        p1: Position2DI32 {
            x: PosXI32(MyUnitI32(2)),
            y: PosYI32(MyUnitI32(3)),
        },
        p2: Position2DI32 {
            x: PosXI32(MyUnitI32(4)),
            y: PosYI32(MyUnitI32(5)),
        },
    };
    assert_eq!(box2d.normalize(), normal_box2d);
}

#[test]
fn split_from_left() {
    let orig_box = Box2DI32::<MyUnitI32> {
        p1: Position2DI32 {
            x: PosXI32(MyUnitI32(2)),
            y: PosYI32(MyUnitI32(3)),
        },
        p2: Position2DI32 {
            x: PosXI32(MyUnitI32(7)),
            y: PosYI32(MyUnitI32(8)),
        },
    };
    let (left_box, right_box) = orig_box.split_from_left(WidthI32(MyUnitI32(2)));
    assert_eq!(left_box.p1, orig_box.p1);
    assert_eq!(right_box.p2, orig_box.p2);
    assert_eq!(left_box.width() + right_box.width(), orig_box.width());
    assert_eq!(left_box.height(), orig_box.height());
    assert_eq!(right_box.height(), orig_box.height());
    assert_eq!(
        left_box,
        Box2DI32 {
            p1: Position2DI32 {
                x: PosXI32(MyUnitI32(2)),
                y: PosYI32(MyUnitI32(3)),
            },
            p2: Position2DI32 {
                x: PosXI32(MyUnitI32(4)),
                y: PosYI32(MyUnitI32(8)),
            },
        }
    );
    assert_eq!(
        right_box,
        Box2DI32 {
            p1: Position2DI32 {
                x: PosXI32(MyUnitI32(4)),
                y: PosYI32(MyUnitI32(3)),
            },
            p2: Position2DI32 {
                x: PosXI32(MyUnitI32(7)),
                y: PosYI32(MyUnitI32(8)),
            },
        }
    );
}
