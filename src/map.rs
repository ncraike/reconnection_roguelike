use bracket_algorithm_traits::prelude::{Algorithm2D, BaseMap};
use bracket_geometry::prelude::{Point, Rect};

pub const MAP_WIDTH: u32 = 80;
pub const MAP_HEIGHT: u32 = 25;

pub const TILE_1X_WIDTH: u32 = 16;
pub const TILE_1X_HEIGHT: u32 = 24;
pub const TILE_2X_WIDTH: u32 = 32;
pub const TILE_2X_HEIGHT: u32 = 48;

#[derive(PartialEq, Copy, Clone)]
pub enum TileGraphic {
    // Ground
    Void,
    Ground1,
    Ground2,
    Ground3,
    Ground4,
    Floor1,
    Floor2,
    // Walls
    WallHExternal = 8,
    WallSECornerExternal,
    WallSWCornerExternal,
    WallSEndExternal,
    WallEEndExternal,
    WallWEndExternal,
    WallNTeeExternal,
    WallHInternal = 16,
    WallSECornerInternal,
    WallSWCornerInternal,
    WallSEndInternal,
    WallEEndInternal,
    WallWEndInternal,
    WallNTeeInternal,
    WallV = 24,
    WallNWCorner,
    WallNECorner,
    WallNEnd,
    WallSTee,
    WallETee,
    WallWTee,
    // Player & NPCs
    PlayerCharacter = 32,
    // Enemies
    EnemyHound = 40,
    EnemySmallStalker,
    EnemyBigStalker,
    // Debug
    DebugOverlay = 48,
}

pub struct Map {
    pub terrain: Vec<TileGraphic>,
    pub revealed_terrain: Vec<bool>,
    pub visible_terrain: Vec<bool>,
    pub width: u32,
    pub height: u32,
}

impl Map {
    pub fn to_index(&self, point: Point) -> usize {
        point.to_index(self.width)
    }

    pub fn bounds(&self) -> Rect {
        Rect::with_size(0, 0, self.width, self.height)
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        // Fill inside
        for y in (room.y1 + 1)..room.y2 {
            for x in (room.x1 + 1)..room.x2 {
                let inside = self.to_index(Point { x, y });
                self.terrain[inside] = TileGraphic::Floor1;
            }
        }

        // Corners
        let nw_corner = self.to_index(Point {
            x: room.x1,
            y: room.y1,
        });
        self.terrain[nw_corner] = TileGraphic::WallNWCorner;
        let ne_corner = self.to_index(Point {
            x: room.x2,
            y: room.y1,
        });
        self.terrain[ne_corner] = TileGraphic::WallNECorner;
        let se_corner = self.to_index(Point {
            x: room.x2,
            y: room.y2,
        });
        self.terrain[se_corner] = TileGraphic::WallSECornerExternal;
        let sw_corner = self.to_index(Point {
            x: room.x1,
            y: room.y2,
        });
        self.terrain[sw_corner] = TileGraphic::WallSWCornerExternal;

        for x in (room.x1 + 1)..room.x2 {
            // Top wall
            let top = self.to_index(Point { x, y: room.y1 });
            self.terrain[top] = TileGraphic::WallHInternal;
            // Bottom wall
            let bottom = self.to_index(Point { x, y: room.y2 });
            self.terrain[bottom] = TileGraphic::WallHExternal;
        }
        for y in (room.y1 + 1)..room.y2 {
            // Left wall
            let left = self.to_index(Point { x: room.x1, y });
            self.terrain[left] = TileGraphic::WallV;
            // Right wall
            let right = self.to_index(Point { x: room.x2, y });
            self.terrain[right] = TileGraphic::WallV;
        }
    }

    pub fn new_map() -> Map {
        let mut map = Map {
            terrain: vec![TileGraphic::Ground1; (MAP_WIDTH * MAP_HEIGHT) as usize],
            revealed_terrain: vec![false; (MAP_WIDTH * MAP_HEIGHT) as usize],
            visible_terrain: vec![false; (MAP_WIDTH * MAP_HEIGHT) as usize],
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
        };

        let room = Rect::with_exact(16, 2, 22, 7);
        map.apply_room_to_map(&room);

        map
    }
}

pub fn is_passable(tile: TileGraphic) -> bool {
    match tile {
        TileGraphic::Void
        | TileGraphic::Ground1
        | TileGraphic::Ground2
        | TileGraphic::Ground3
        | TileGraphic::Ground4
        | TileGraphic::Floor1
        | TileGraphic::Floor2 => true,
        _ => false,
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, _idx: usize) -> bool {
        !is_passable(self.terrain[_idx])
    }
}
