use bracket_geometry::prelude::*;
use bracket_terminal::prelude::*;

pub const WIDTH: i32 = 25;
pub const HEIGHT: i32 = 14;

pub const TILE_WIDTH: u32 = 32;
pub const TILE_HEIGHT: u32 = 32;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Ground,
    Floor,
    Wall,
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileGraphic {
    Ground=0,
    InsideFloor,
    LRGroundInside=8,
    LRInsideGround,
    TBGroundInside,
    TBInsideGround,
    TLGroundInside,
    TRGroundInside,
    BRGroundInside,
    BLGroundInside,
    TLInsideGround,
    TRInsideGround,
    BRInsideGround,
    BLInsideGround,
    WallV=24,
    WallHExternal,
    WallTLCornerExternal,
    WallTRCornerExternal,
    WallBRCornerExternal,
    WallBLCornerExternal,
    WallHInternal,
    WallTLCornerInternal,
    WallTRCornerInternal,
    WallBRCornerInternal,
    WallBLCornerInternal,
    PlayerCharacter=56,
    PlayerCharacterWithBackpack,
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileLayer {
    Floor,
    Walls,
    Decorations,
    NPCs,
    PlayerCharacter,
}


pub fn xy_idx(x: i32, y: i32) -> usize {
    ((y * WIDTH) + x) as usize
}


pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Ground; (WIDTH * HEIGHT) as usize];

    const ROOM_P1: Point = Point{ x: 16, y: 2};
    const ROOM_P2: Point = Point{ x: 22, y: 7};

    for x in ROOM_P1.x..=ROOM_P2.x {
        map[xy_idx(x, ROOM_P1.y)] = TileType::Wall;
        map[xy_idx(x, ROOM_P2.y)] = TileType::Wall;
    }

    for y in ROOM_P1.y..=ROOM_P2.y {
        map[xy_idx(ROOM_P1.x, y)] = TileType::Wall;
        map[xy_idx(ROOM_P2.x, y)] = TileType::Wall;
    }

    map
}

pub fn draw_map(map: &[TileType]) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let mut x = 0;
    let mut y = 0;
    for tile in map.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Ground => {
                draw_batch.set(
                    Point { x: x, y: y},
                    ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0., 0., 0.)),
                    TileGraphic::Ground as u16,
                );
            }
            TileType::Floor => {
                draw_batch.set(
                    Point { x: x, y: y},
                    ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0., 0., 0.)),
                    TileGraphic::InsideFloor as u16,
                );
            }
            TileType::Wall => {
                draw_batch.set(
                    Point { x: x, y: y},
                    ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0., 0., 0.)),
                    TileGraphic::WallHExternal as u16,
                );
            }
        }

        // Move the coordinates
        x += 1;
        if x > (WIDTH - 1) {
            x = 0;
            y += 1;
        }
    }
}
