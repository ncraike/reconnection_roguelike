use super::base::MyUnitI32;
use crate::{Box2DI32, HeightI32, PosXI32, PosYI32, Position2DI32, Size2DI32, UnitI32, WidthI32};

#[test]
fn box2d_x1_y1_x2_y2() {
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
fn box2d_size_width_height() {
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
