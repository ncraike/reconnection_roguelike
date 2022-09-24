extern crate bracket_geometry;
use bracket_geometry::prelude::*;
extern crate bracket_terminal;
use bracket_terminal::prelude::*;
extern crate specs;
use specs::prelude::*;

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
    EmptyFloor=0,
    Ground,
    InsideFloor,
    EmptyFloorVariant=8,
    LRGroundInside,
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
    EmptyWall=24,
    WallV,
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
    EmptyPlayerCharacter=56,
    PlayerCharacter,
    PlayerCharacterWithBackpack,
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileLayer {
    Floor,
    Walls,
    NPCs,
    Decorations,
    PlayerCharacter,
}

pub struct Map {
    pub floor_tiles: Vec<TileGraphic>,
    pub wall_tiles: Vec<TileGraphic>,
    pub width: i32,
    pub height: i32
}

impl Map {
    pub fn to_index(&self, point: Point) -> usize {
        point.to_index(self.width)
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        // Fill inside
        for y in (room.y1 + 1) .. room.y2 {
            for x in (room.x1 + 1) .. room.x2 {
                let inside = self.to_index(Point{x, y});
                self.floor_tiles[inside] = TileGraphic::InsideFloor;
                self.wall_tiles[inside] = TileGraphic::EmptyWall;
            }
        }

        // Top-left corner
        let top_left = self.to_index(Point{x: room.x1, y: room.y1});
        self.floor_tiles[top_left] = TileGraphic::TLGroundInside;
        self.wall_tiles[top_left] = TileGraphic::WallTLCornerInternal;
        // Top-right corner
        let top_right = self.to_index(Point{x: room.x2, y: room.y1});
        self.floor_tiles[top_right] = TileGraphic::TRGroundInside;
        self.wall_tiles[top_right] = TileGraphic::WallTRCornerInternal;
        // Bottom-right corner
        let bottom_right = self.to_index(Point{x: room.x2, y: room.y2});
        self.floor_tiles[bottom_right] = TileGraphic::BRGroundInside;
        self.wall_tiles[bottom_right] = TileGraphic::WallBRCornerExternal;
        // Bottom-left corner
        let bottom_left = self.to_index(Point{x: room.x1, y: room.y2});
        self.floor_tiles[bottom_left] = TileGraphic::BLGroundInside;
        self.wall_tiles[bottom_left] = TileGraphic::WallBLCornerExternal;

        for x in (room.x1 + 1) .. room.x2 {
            // Top wall
            let top = self.to_index(Point{x, y: room.y1});
            self.floor_tiles[top] = TileGraphic::TBGroundInside;
            self.wall_tiles[top] = TileGraphic::WallHInternal;
            // Bottom wall
            let bottom = self.to_index(Point{x, y: room.y2});
            self.floor_tiles[bottom] = TileGraphic::TBInsideGround;
            self.wall_tiles[bottom] = TileGraphic::WallHExternal;
        }
        for y in (room.y1 + 1) .. room.y2 {
            // Left wall
            let left = self.to_index(Point{x: room.x1, y});
            self.floor_tiles[left] = TileGraphic::LRGroundInside;
            self.wall_tiles[left] = TileGraphic::WallV;
            // Right wall
            let right = self.to_index(Point{x: room.x2, y});
            self.floor_tiles[right] = TileGraphic::LRInsideGround;
            self.wall_tiles[right] = TileGraphic::WallV;
        }
    }

    pub fn new_map() -> Map {
        let mut map = Map{
            floor_tiles: vec![TileGraphic::Ground; (WIDTH*HEIGHT) as usize],
            wall_tiles: vec![TileGraphic::EmptyWall; (WIDTH*HEIGHT) as usize],
            width: WIDTH,
            height: HEIGHT
        };

        let room = Rect::with_exact(16, 2, 22, 7);
        map.apply_room_to_map(&room);

        map
    }
}


pub fn draw_map(ecs: &World) {
    let map = ecs.fetch::<Map>();

    let map_area = Rect::with_size(0, 0, map.width, map.height);
    let mut floor_draws = DrawBatch::new();
    floor_draws.target(1);
    let mut wall_draws = DrawBatch::new();
    wall_draws.target(1);

    let solid_color: ColorPair = ColorPair::new(
        RGB::from_f32(1.0, 1.0, 1.0),
        RGB::from_f32(0., 0., 0.),
    );

    for point in map_area.point_set().iter() {
        let point_index = map.to_index(*point);
        floor_draws.set(*point, solid_color, map.floor_tiles[point_index] as u16);
        wall_draws.set(*point, solid_color, map.wall_tiles[point_index] as u16);
    }

    floor_draws.submit(TileLayer::Floor as usize).expect("Failed to draw floors");
    wall_draws.submit(TileLayer::Walls as usize).expect("Failed to draw walls");
}
