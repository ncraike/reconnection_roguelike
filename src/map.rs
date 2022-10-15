use bracket_color::prelude::{ColorPair, RGB};
use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::DrawBatch;
use specs::prelude::*;

pub const WIDTH: i32 = 25;
pub const HEIGHT: i32 = 14;

pub const TILE_WIDTH: u32 = 16 * 2;
pub const TILE_HEIGHT: u32 = 24 * 2;

#[derive(PartialEq, Copy, Clone)]
pub enum TileGraphic {
    Void,
    Ground1,
    Ground2,
    Ground3,
    Ground4,
    Floor1,
    Floor2,
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
    PlayerCharacter = 32,
}

pub struct Map {
    pub terrain: Vec<TileGraphic>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn to_index(&self, point: Point) -> usize {
        point.to_index(self.width)
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
            terrain: vec![TileGraphic::Ground1; (WIDTH * HEIGHT) as usize],
            width: WIDTH,
            height: HEIGHT,
        };

        let room = Rect::with_exact(16, 2, 22, 7);
        map.apply_room_to_map(&room);

        map
    }
}

pub fn draw_map(ecs: &World) {
    let map = ecs.fetch::<Map>();

    let map_area = Rect::with_size(0, 0, map.width, map.height);
    let mut draws = DrawBatch::new();
    draws.cls();
    draws.target(0);

    let solid_color: ColorPair =
        ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(1.0, 1.0, 1.0));

    for point in map_area.point_set().iter() {
        let point_index = map.to_index(*point);
        draws.set(*point, solid_color, map.terrain[point_index] as u16);
    }

    draws.submit(0).expect("Failed to draw walls");
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
