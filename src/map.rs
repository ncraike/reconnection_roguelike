use bracket_algorithm_traits::prelude::{Algorithm2D, BaseMap, SmallVec};
use bracket_geometry::prelude::{DistanceAlg, Point, Rect};
use specs::prelude::*;

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
    // Items
    ItemBandage = 48,
    ItemFirstAidKit,
    // Debug
    DebugOverlay = 56,
}

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<TileGraphic>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    pub fn new_map() -> Map {
        let mut map = Map {
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            tiles: vec![TileGraphic::Ground1; (MAP_WIDTH * MAP_HEIGHT) as usize],
            revealed_tiles: vec![false; (MAP_WIDTH * MAP_HEIGHT) as usize],
            visible_tiles: vec![false; (MAP_WIDTH * MAP_HEIGHT) as usize],
            blocked: vec![false; (MAP_WIDTH * MAP_HEIGHT) as usize],
            tile_content: vec![Vec::new(); (MAP_WIDTH * MAP_HEIGHT) as usize],
        };

        let room = Rect::with_exact(16, 2, 22, 7);
        map.apply_room_to_map(&room);

        map
    }

    pub fn to_index(&self, point: Point) -> usize {
        point.to_index(self.width)
    }

    pub fn to_point(&self, index: usize) -> Point {
        Point {
            x: (index as i32) % (self.width as i32),
            y: (index as i32) / (self.width as i32),
        }
    }

    pub fn bounds(&self) -> Rect {
        Rect::with_size(0, 0, self.width, self.height)
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        // Fill inside
        for y in (room.y1 + 1)..room.y2 {
            for x in (room.x1 + 1)..room.x2 {
                let inside = self.to_index(Point { x, y });
                self.tiles[inside] = TileGraphic::Floor1;
            }
        }

        // Corners
        let nw_corner = self.to_index(Point {
            x: room.x1,
            y: room.y1,
        });
        self.tiles[nw_corner] = TileGraphic::WallNWCorner;
        let ne_corner = self.to_index(Point {
            x: room.x2,
            y: room.y1,
        });
        self.tiles[ne_corner] = TileGraphic::WallNECorner;
        let se_corner = self.to_index(Point {
            x: room.x2,
            y: room.y2,
        });
        self.tiles[se_corner] = TileGraphic::WallSECornerExternal;
        let sw_corner = self.to_index(Point {
            x: room.x1,
            y: room.y2,
        });
        self.tiles[sw_corner] = TileGraphic::WallSWCornerExternal;

        for x in (room.x1 + 1)..room.x2 {
            // Top wall
            let top = self.to_index(Point { x, y: room.y1 });
            self.tiles[top] = TileGraphic::WallHInternal;
            // Bottom wall
            let bottom = self.to_index(Point { x, y: room.y2 });
            self.tiles[bottom] = TileGraphic::WallHExternal;
        }
        for y in (room.y1 + 1)..room.y2 {
            // Left wall
            let left = self.to_index(Point { x: room.x1, y });
            self.tiles[left] = TileGraphic::WallV;
            // Right wall
            let right = self.to_index(Point { x: room.x2, y });
            self.tiles[right] = TileGraphic::WallV;
        }
    }

    pub fn can_move_to(&self, point: Point) -> bool {
        self.in_bounds(point) && !self.blocked[self.to_index(point)]
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter().enumerate() {
            self.blocked[i] = !is_passable(*tile);
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
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
        !is_passable(self.tiles[_idx])
    }

    fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let point = self.to_point(_idx);

        let directions = vec![
            (0, -1, 1.0),   // north
            (0, 1, 1.0),    // south
            (1, 0, 1.0),    // east
            (-1, 0, 1.0),   // west
            (-1, -1, 1.45), // north-west
            (1, -1, 1.45),  // north-east
            (-1, 1, 1.45),  // south-west
            (1, 1, 1.45),   // south-east
        ];
        for direction in directions {
            let (dir_x, dir_y, distance) = direction;
            let candidate_exit = point + Point { x: dir_x, y: dir_y };
            if self.can_move_to(candidate_exit) {
                exits.push((self.to_index(candidate_exit), distance))
            }
        }

        exits
    }

    fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
        let point1 = self.to_point(_idx1);
        let point2 = self.to_point(_idx2);
        DistanceAlg::Pythagoras.distance2d(point1, point2)
    }
}
