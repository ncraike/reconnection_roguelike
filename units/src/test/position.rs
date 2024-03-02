use super::base::MyUnitI32;
use crate::{Height, PosX, PosY, Position2D, Size2D, Width};

#[test]
fn pos_y_i32_add_height_gives_pos_y() {
    let y = PosY(MyUnitI32(2));
    let h = Height(MyUnitI32(3));
    assert_eq!(y + h, PosY::<MyUnitI32>(MyUnitI32(5)));
}

#[test]
fn pos_y_i32_sub_pos_y_gives_height() {
    let y1 = PosY(MyUnitI32(2));
    let y2 = PosY(MyUnitI32(5));
    assert_eq!(y2 - y1, Height::<MyUnitI32>(MyUnitI32(3)));
}

#[test]
fn position_2d_i32_add_size2d_gives_position2d() {
    let pos2d = Position2D::<MyUnitI32> {
        x: PosX(MyUnitI32(2)),
        y: PosY(MyUnitI32(3)),
    };
    let size2d = Size2D::<MyUnitI32> {
        width: Width(MyUnitI32(4)),
        height: Height(MyUnitI32(5)),
    };
    assert_eq!(
        pos2d + size2d,
        Position2D::<MyUnitI32> {
            x: PosX(MyUnitI32(6)),
            y: PosY(MyUnitI32(8)),
        }
    );
}

#[test]
fn position_2d_i32_sub_position_2d_gives_size_2d() {
    let pos1 = Position2D::<MyUnitI32> {
        x: PosX(MyUnitI32(2)),
        y: PosY(MyUnitI32(3)),
    };
    let pos2 = Position2D::<MyUnitI32> {
        x: PosX(MyUnitI32(6)),
        y: PosY(MyUnitI32(8)),
    };
    assert_eq!(
        pos2 - pos1,
        Size2D::<MyUnitI32> {
            width: Width(MyUnitI32(4)),
            height: Height(MyUnitI32(5)),
        }
    );
}

#[test]
fn position_2d_i32_add_width_gives_position_2d() {
    let pos2d = Position2D::<MyUnitI32> {
        x: PosX(MyUnitI32(2)),
        y: PosY(MyUnitI32(3)),
    };
    let width = Width(MyUnitI32(4));
    assert_eq!(
        pos2d + width,
        Position2D::<MyUnitI32> {
            x: PosX(MyUnitI32(6)),
            y: PosY(MyUnitI32(3)),
        }
    );
}

#[test]
fn position_2d_i32_sub_width_gives_position_2d() {
    let pos2d = Position2D::<MyUnitI32> {
        x: PosX(MyUnitI32(6)),
        y: PosY(MyUnitI32(3)),
    };
    let width = Width(MyUnitI32(4));
    assert_eq!(
        pos2d - width,
        Position2D::<MyUnitI32> {
            x: PosX(MyUnitI32(2)),
            y: PosY(MyUnitI32(3)),
        }
    );
}

#[test]
fn position_2d_i32_add_height_gives_position_2d() {
    let pos2d = Position2D::<MyUnitI32> {
        x: PosX(MyUnitI32(2)),
        y: PosY(MyUnitI32(3)),
    };
    let height = Height(MyUnitI32(5));
    assert_eq!(
        pos2d + height,
        Position2D::<MyUnitI32> {
            x: PosX(MyUnitI32(2)),
            y: PosY(MyUnitI32(8)),
        }
    );
}

#[test]
fn position_2d_i32_sub_height_gives_position_2d() {
    let pos2d = Position2D::<MyUnitI32> {
        x: PosX(MyUnitI32(2)),
        y: PosY(MyUnitI32(8)),
    };
    let height = Height(MyUnitI32(5));
    assert_eq!(
        pos2d - height,
        Position2D::<MyUnitI32> {
            x: PosX(MyUnitI32(2)),
            y: PosY(MyUnitI32(3)),
        }
    );
}
