use super::base::MyUnitI32;
use crate::{Box2D, Height, PosX, PosY, Position2D, Size2D, Width};

#[test]
fn x1_y1_x2_y2() {
    let box2d = Box2D::<MyUnitI32> {
        p1: Position2D {
            x: PosX(MyUnitI32(2)),
            y: PosY(MyUnitI32(3)),
        },
        p2: Position2D {
            x: PosX(MyUnitI32(4)),
            y: PosY(MyUnitI32(5)),
        },
    };
    assert_eq!(box2d.x1(), PosX(MyUnitI32(2)));
    assert_eq!(box2d.y1(), PosY(MyUnitI32(3)));
    assert_eq!(box2d.x2(), PosX(MyUnitI32(4)));
    assert_eq!(box2d.y2(), PosY(MyUnitI32(5)));
}

#[test]
fn size_width_height() {
    let box2d = Box2D::<MyUnitI32> {
        p1: Position2D {
            x: PosX(MyUnitI32(2)),
            y: PosY(MyUnitI32(3)),
        },
        p2: Position2D {
            x: PosX(MyUnitI32(4)),
            y: PosY(MyUnitI32(6)),
        },
    };
    assert_eq!(
        box2d.size(),
        Size2D {
            width: Width(MyUnitI32(2)),
            height: Height(MyUnitI32(3)),
        }
    );
    assert_eq!(box2d.width(), Width(MyUnitI32(2)));
    assert_eq!(box2d.height(), Height(MyUnitI32(3)));
}

#[test]
fn normalize_does_not_change_already_normal() {
    let box2d = Box2D::<MyUnitI32> {
        p1: Position2D {
            x: PosX(MyUnitI32(2)),
            y: PosY(MyUnitI32(3)),
        },
        p2: Position2D {
            x: PosX(MyUnitI32(4)),
            y: PosY(MyUnitI32(5)),
        },
    };
    assert_eq!(box2d.normalize(), box2d);
}

#[test]
fn normalize_swaps_p1_p2() {
    let box2d = Box2D::<MyUnitI32> {
        p1: Position2D {
            x: PosX(MyUnitI32(4)),
            y: PosY(MyUnitI32(5)),
        },
        p2: Position2D {
            x: PosX(MyUnitI32(2)),
            y: PosY(MyUnitI32(3)),
        },
    };
    let normal_box2d = Box2D::<MyUnitI32> {
        p1: Position2D {
            x: PosX(MyUnitI32(2)),
            y: PosY(MyUnitI32(3)),
        },
        p2: Position2D {
            x: PosX(MyUnitI32(4)),
            y: PosY(MyUnitI32(5)),
        },
    };
    assert_eq!(box2d.normalize(), normal_box2d);
}

#[test]
fn normalize_picks_top_left_bottom_right_corners() {
    let box2d = Box2D::<MyUnitI32> {
        // Bottom-left corner (assuming top-left origin)
        p1: Position2D {
            x: PosX(MyUnitI32(2)),
            y: PosY(MyUnitI32(5)),
        },
        // Top-right corner (assuming top-left origin)
        p2: Position2D {
            x: PosX(MyUnitI32(4)),
            y: PosY(MyUnitI32(3)),
        },
    };
    let normal_box2d = Box2D::<MyUnitI32> {
        p1: Position2D {
            x: PosX(MyUnitI32(2)),
            y: PosY(MyUnitI32(3)),
        },
        p2: Position2D {
            x: PosX(MyUnitI32(4)),
            y: PosY(MyUnitI32(5)),
        },
    };
    assert_eq!(box2d.normalize(), normal_box2d);
}

#[test]
fn split_from_left() {
    let orig_box = Box2D::<MyUnitI32> {
        p1: Position2D {
            x: PosX(MyUnitI32(2)),
            y: PosY(MyUnitI32(3)),
        },
        p2: Position2D {
            x: PosX(MyUnitI32(7)),
            y: PosY(MyUnitI32(8)),
        },
    };
    let (left_box, right_box) = orig_box.split_from_left(Width(MyUnitI32(2)));
    assert_eq!(left_box.p1, orig_box.p1);
    assert_eq!(right_box.p2, orig_box.p2);
    assert_eq!(left_box.width(), Width(MyUnitI32(2)));
    assert_eq!(left_box.width() + right_box.width(), orig_box.width());
    assert_eq!(left_box.height(), orig_box.height());
    assert_eq!(right_box.height(), orig_box.height());
    assert_eq!(
        left_box,
        Box2D {
            p1: Position2D {
                x: PosX(MyUnitI32(2)),
                y: PosY(MyUnitI32(3)),
            },
            p2: Position2D {
                x: PosX(MyUnitI32(4)),
                y: PosY(MyUnitI32(8)),
            },
        }
    );
    assert_eq!(
        right_box,
        Box2D {
            p1: Position2D {
                x: PosX(MyUnitI32(4)),
                y: PosY(MyUnitI32(3)),
            },
            p2: Position2D {
                x: PosX(MyUnitI32(7)),
                y: PosY(MyUnitI32(8)),
            },
        }
    );
}

#[test]
fn split_from_right() {
    let orig_box = Box2D::<MyUnitI32> {
        p1: Position2D {
            x: PosX(MyUnitI32(2)),
            y: PosY(MyUnitI32(3)),
        },
        p2: Position2D {
            x: PosX(MyUnitI32(7)),
            y: PosY(MyUnitI32(8)),
        },
    };
    let (left_box, right_box) = orig_box.split_from_right(Width(MyUnitI32(2)));
    assert_eq!(left_box.p1, orig_box.p1);
    assert_eq!(right_box.p2, orig_box.p2);
    assert_eq!(right_box.width(), Width(MyUnitI32(2)));
    assert_eq!(left_box.width() + right_box.width(), orig_box.width());
    assert_eq!(left_box.height(), orig_box.height());
    assert_eq!(right_box.height(), orig_box.height());
    assert_eq!(
        left_box,
        Box2D {
            p1: Position2D {
                x: PosX(MyUnitI32(2)),
                y: PosY(MyUnitI32(3)),
            },
            p2: Position2D {
                x: PosX(MyUnitI32(5)),
                y: PosY(MyUnitI32(8)),
            },
        }
    );
    assert_eq!(
        right_box,
        Box2D {
            p1: Position2D {
                x: PosX(MyUnitI32(5)),
                y: PosY(MyUnitI32(3)),
            },
            p2: Position2D {
                x: PosX(MyUnitI32(7)),
                y: PosY(MyUnitI32(8)),
            },
        }
    );
}

#[test]
fn split_from_top() {
    let orig_box = Box2D::<MyUnitI32> {
        p1: Position2D {
            x: PosX(MyUnitI32(2)),
            y: PosY(MyUnitI32(3)),
        },
        p2: Position2D {
            x: PosX(MyUnitI32(7)),
            y: PosY(MyUnitI32(9)),
        },
    };
    let (top_box, bottom_box) = orig_box.split_from_top(Height(MyUnitI32(2)));
    assert_eq!(top_box.p1, orig_box.p1);
    assert_eq!(bottom_box.p2, orig_box.p2);
    assert_eq!(top_box.height(), Height(MyUnitI32(2)));
    assert_eq!(top_box.height() + bottom_box.height(), orig_box.height());
    assert_eq!(top_box.width(), orig_box.width());
    assert_eq!(bottom_box.width(), orig_box.width());
    assert_eq!(
        top_box,
        Box2D {
            p1: Position2D {
                x: PosX(MyUnitI32(2)),
                y: PosY(MyUnitI32(3)),
            },
            p2: Position2D {
                x: PosX(MyUnitI32(7)),
                y: PosY(MyUnitI32(5)),
            },
        }
    );
    assert_eq!(
        bottom_box,
        Box2D {
            p1: Position2D {
                x: PosX(MyUnitI32(2)),
                y: PosY(MyUnitI32(5)),
            },
            p2: Position2D {
                x: PosX(MyUnitI32(7)),
                y: PosY(MyUnitI32(9)),
            },
        }
    );
}

#[test]
fn split_from_bottom() {
    let orig_box = Box2D::<MyUnitI32> {
        p1: Position2D {
            x: PosX(MyUnitI32(2)),
            y: PosY(MyUnitI32(3)),
        },
        p2: Position2D {
            x: PosX(MyUnitI32(7)),
            y: PosY(MyUnitI32(9)),
        },
    };
    let (top_box, bottom_box) = orig_box.split_from_bottom(Height(MyUnitI32(2)));
    assert_eq!(top_box.p1, orig_box.p1);
    assert_eq!(bottom_box.p2, orig_box.p2);
    assert_eq!(bottom_box.height(), Height(MyUnitI32(2)));
    assert_eq!(top_box.height() + bottom_box.height(), orig_box.height());
    assert_eq!(top_box.width(), orig_box.width());
    assert_eq!(bottom_box.width(), orig_box.width());
    assert_eq!(
        top_box,
        Box2D {
            p1: Position2D {
                x: PosX(MyUnitI32(2)),
                y: PosY(MyUnitI32(3)),
            },
            p2: Position2D {
                x: PosX(MyUnitI32(7)),
                y: PosY(MyUnitI32(7)),
            },
        }
    );
    assert_eq!(
        bottom_box,
        Box2D {
            p1: Position2D {
                x: PosX(MyUnitI32(2)),
                y: PosY(MyUnitI32(7)),
            },
            p2: Position2D {
                x: PosX(MyUnitI32(7)),
                y: PosY(MyUnitI32(9)),
            },
        }
    );
}
