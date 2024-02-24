use super::base::MyUnitI32;
use crate::{HeightI32, PosXI32, PosYI32, Position2DI32, Size2DI32, WidthI32};

#[test]
fn pos_x_i32_add_width_gives_pos_x() {
    let x = PosXI32(MyUnitI32(3));
    let w = WidthI32(MyUnitI32(4));
    assert_eq!(x + w, PosXI32::<MyUnitI32>(MyUnitI32(7)));
}

#[test]
fn pos_x_i32_sub_pos_x_gives_width() {
    let x1 = PosXI32(MyUnitI32(3));
    let x2 = PosXI32(MyUnitI32(7));
    assert_eq!(x2 - x1, WidthI32::<MyUnitI32>(MyUnitI32(4)));
}

#[test]
fn pos_y_i32_add_height_gives_pos_y() {
    let y = PosYI32(MyUnitI32(2));
    let h = HeightI32(MyUnitI32(3));
    assert_eq!(y + h, PosYI32::<MyUnitI32>(MyUnitI32(5)));
}

#[test]
fn pos_y_i32_sub_pos_y_gives_height() {
    let y1 = PosYI32(MyUnitI32(2));
    let y2 = PosYI32(MyUnitI32(5));
    assert_eq!(y2 - y1, HeightI32::<MyUnitI32>(MyUnitI32(3)));
}

#[test]
fn position_2d_i32_add_size2d_gives_position2d() {
    let pos2d = Position2DI32::<MyUnitI32> {
        x: PosXI32(MyUnitI32(2)),
        y: PosYI32(MyUnitI32(3)),
    };
    let size2d = Size2DI32::<MyUnitI32> {
        width: WidthI32(MyUnitI32(4)),
        height: HeightI32(MyUnitI32(5)),
    };
    assert_eq!(
        pos2d + size2d,
        Position2DI32::<MyUnitI32> {
            x: PosXI32(MyUnitI32(6)),
            y: PosYI32(MyUnitI32(8)),
        }
    );
}

#[test]
fn position_2d_i32_sub_position_2d_gives_size_2d() {
    let pos1 = Position2DI32::<MyUnitI32> {
        x: PosXI32(MyUnitI32(2)),
        y: PosYI32(MyUnitI32(3)),
    };
    let pos2 = Position2DI32::<MyUnitI32> {
        x: PosXI32(MyUnitI32(6)),
        y: PosYI32(MyUnitI32(8)),
    };
    assert_eq!(
        pos2 - pos1,
        Size2DI32::<MyUnitI32> {
            width: WidthI32(MyUnitI32(4)),
            height: HeightI32(MyUnitI32(5)),
        }
    );
}

#[test]
fn position_2d_i32_add_width_gives_position_2d() {
    let pos2d = Position2DI32::<MyUnitI32> {
        x: PosXI32(MyUnitI32(2)),
        y: PosYI32(MyUnitI32(3)),
    };
    let width = WidthI32(MyUnitI32(4));
    assert_eq!(
        pos2d + width,
        Position2DI32::<MyUnitI32> {
            x: PosXI32(MyUnitI32(6)),
            y: PosYI32(MyUnitI32(3)),
        }
    );
}

#[test]
fn position_2d_i32_sub_width_gives_position_2d() {
    let pos2d = Position2DI32::<MyUnitI32> {
        x: PosXI32(MyUnitI32(6)),
        y: PosYI32(MyUnitI32(3)),
    };
    let width = WidthI32(MyUnitI32(4));
    assert_eq!(
        pos2d - width,
        Position2DI32::<MyUnitI32> {
            x: PosXI32(MyUnitI32(2)),
            y: PosYI32(MyUnitI32(3)),
        }
    );
}

#[test]
fn position_2d_i32_add_height_gives_position_2d() {
    let pos2d = Position2DI32::<MyUnitI32> {
        x: PosXI32(MyUnitI32(2)),
        y: PosYI32(MyUnitI32(3)),
    };
    let height = HeightI32(MyUnitI32(5));
    assert_eq!(
        pos2d + height,
        Position2DI32::<MyUnitI32> {
            x: PosXI32(MyUnitI32(2)),
            y: PosYI32(MyUnitI32(8)),
        }
    );
}

#[test]
fn position_2d_i32_sub_height_gives_position_2d() {
    let pos2d = Position2DI32::<MyUnitI32> {
        x: PosXI32(MyUnitI32(2)),
        y: PosYI32(MyUnitI32(8)),
    };
    let height = HeightI32(MyUnitI32(5));
    assert_eq!(
        pos2d - height,
        Position2DI32::<MyUnitI32> {
            x: PosXI32(MyUnitI32(2)),
            y: PosYI32(MyUnitI32(3)),
        }
    );
}
