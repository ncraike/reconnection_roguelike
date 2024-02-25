use bracket_algorithm_traits::prelude::{Algorithm2D, BaseMap, SmallVec};
use bracket_geometry::prelude::{DistanceAlg, Point};
use specs::prelude::*;
use units::{Box2DI32, PosXI32, PosYI32, Position2DI32, Size2DI32};

use crate::world::units::WorldUnits;

pub const MAP_WIDTH: u32 = 80;
pub const MAP_HEIGHT: u32 = 25;

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
    pub size: Size2DI32<WorldUnits>,
    pub tiles: Vec<TileGraphic>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    pub fn new_map() -> Map {
        let mut map = Map {
            size: WorldUnits::new_size2d(MAP_WIDTH as i32, MAP_HEIGHT as i32),
            tiles: vec![TileGraphic::Ground1; (MAP_WIDTH * MAP_HEIGHT) as usize],
            revealed_tiles: vec![false; (MAP_WIDTH * MAP_HEIGHT) as usize],
            visible_tiles: vec![false; (MAP_WIDTH * MAP_HEIGHT) as usize],
            blocked: vec![false; (MAP_WIDTH * MAP_HEIGHT) as usize],
            tile_content: vec![Vec::new(); (MAP_WIDTH * MAP_HEIGHT) as usize],
        };

        let room = WorldUnits::new_box2d_from_x1_y1_x2_y2(16, 2, 22, 7);
        map.apply_room_to_map(&room);

        map
    }

    pub fn to_index(&self, position: Position2DI32<WorldUnits>) -> usize {
        position.to_buffer_index(self.size.width)
    }

    pub fn to_position(&self, index: usize) -> Position2DI32<WorldUnits> {
        Position2DI32::<WorldUnits>::from_buffer_index(index, self.size.width)
    }

    pub fn bounds(&self) -> Box2DI32<WorldUnits> {
        WorldUnits::new_box2d_from_size(self.size)
    }

    fn apply_room_to_map(&mut self, room: &Box2DI32<WorldUnits>) {
        // Fill inside
        for y in (room.y1().to_primitive() + 1)..room.y2().to_primitive() {
            for x in (room.x1().to_primitive() + 1)..room.x2().to_primitive() {
                let inside = self.to_index(WorldUnits::new_position2d(x, y));
                self.tiles[inside] = TileGraphic::Floor1;
            }
        }
        let nw_corner_index = self.to_index(room.p1);
        let ne_corner_index = self.to_index(room.p1.with_x_of(room.p2));
        let se_corner_index = self.to_index(room.p2);
        let sw_corner_index = self.to_index(room.p2.with_x_of(room.p1));

        // Corners
        self.tiles[nw_corner_index] = TileGraphic::WallNWCorner;
        self.tiles[ne_corner_index] = TileGraphic::WallNECorner;
        self.tiles[se_corner_index] = TileGraphic::WallSECornerExternal;
        self.tiles[sw_corner_index] = TileGraphic::WallSWCornerExternal;

        for x in (room.x1().to_primitive() + 1)..room.x2().to_primitive() {
            // Top wall
            let top = self.to_index(Position2DI32 {
                x: PosXI32(WorldUnits(x)),
                y: room.y1(),
            });
            self.tiles[top] = TileGraphic::WallHInternal;
            // Bottom wall
            let bottom = self.to_index(Position2DI32 {
                x: PosXI32(WorldUnits(x)),
                y: room.y2(),
            });
            self.tiles[bottom] = TileGraphic::WallHExternal;
        }
        for y in (room.y1().to_primitive() + 1)..room.y2().to_primitive() {
            // Left wall
            let left = self.to_index(Position2DI32 {
                x: room.x1(),
                y: PosYI32(WorldUnits(y)),
            });
            self.tiles[left] = TileGraphic::WallV;
            // Right wall
            let right = self.to_index(Position2DI32 {
                x: room.x2(),
                y: PosYI32(WorldUnits(y)),
            });
            self.tiles[right] = TileGraphic::WallV;
        }
    }

    pub fn can_move_to(&self, position: Position2DI32<WorldUnits>) -> bool {
        self.in_bounds(position.to_bracket_geometry_point())
            && !self.blocked[self.to_index(position)]
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
        Point::new(
            self.size.width.to_primitive(),
            self.size.height.to_primitive(),
        )
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, _idx: usize) -> bool {
        !is_passable(self.tiles[_idx])
    }

    fn get_available_exits(&self, _idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let point = self.to_position(_idx);

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
            let candidate_exit = point + WorldUnits::new_size2d(dir_x, dir_y);
            if self.can_move_to(candidate_exit) {
                exits.push((self.to_index(candidate_exit), distance))
            }
        }

        exits
    }

    fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
        let pos1 = self.to_position(_idx1);
        let pos2 = self.to_position(_idx2);
        DistanceAlg::Pythagoras.distance2d(
            pos1.to_bracket_geometry_point(),
            pos2.to_bracket_geometry_point(),
        )
    }
}
