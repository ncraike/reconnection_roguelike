use bracket_pathfinding::prelude::*;
use bracket_terminal::prelude::*;
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

bracket_terminal::embedded_resource!(TILE_FONT, "../resources/settlement.png");

const WIDTH: i32 = 25;
const HEIGHT: i32 = 14;
const TILE_WIDTH: u32 = 32;
const TILE_HEIGHT: u32 = 32;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Ground,
    Floor,
    Wall,
}

#[derive(PartialEq, Copy, Clone)]
enum TileGraphic {
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
enum TileLayer {
    Floor,
    Walls,
    Decorations,
    NPCs,
    PlayerCharacter,
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    graphic: TileGraphic,
    layer: TileLayer,
}

#[derive(Component, Debug)]
struct Player {}

struct State {
    ecs: World,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    ((y * WIDTH) + x) as usize
}


fn new_map() -> Vec<TileType> {
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

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(WIDTH - 1, max(0, pos.x + delta_x));
            pos.y = min(HEIGHT - 1, max(0, pos.y + delta_y));
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut BTerm) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            // Laptop controls

            // vim-style HJKL
            VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),
            // diagonals on YUBN
            VirtualKeyCode::Y => try_move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::U => try_move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),
            VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),

            // Arrow keys
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),

            _ => {}
        },
    }
}


fn draw_map(map: &[TileType]) {
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

// Implement the game loop
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        player_input(self, ctx);
        self.run_systems();
    
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map);

        let mut draw_batch = DrawBatch::new();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            draw_batch.target(1);
            draw_batch.cls();
            draw_batch.set(
                Point { x: pos.x, y: pos.y},
                ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0., 0., 0.)),
                render.graphic as u16,
            );
            draw_batch.submit(render.layer as usize).expect("Batch error");
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

fn main() -> BError {
    bracket_terminal::link_resource!(TILE_FONT, "resources/settlement.png");

    let context = BTermBuilder::new()
        // We specify the CONSOLE dimensions
        .with_dimensions(WIDTH, HEIGHT)
        // We specify the size of the tiles
        .with_tile_dimensions(TILE_WIDTH * 2, TILE_HEIGHT * 2)
        // We give it a window title
        .with_title("Reconnection - Settlement")
        // We register our embedded "settlement.png" as a font.
        .with_font("settlement.png", TILE_WIDTH, TILE_HEIGHT)
        // We want a base simple console for the floor layer
        .with_simple_console(WIDTH as u32, HEIGHT as u32, "settlement.png")
        // We also want a sparse console for the layers above
        .with_sparse_console_no_bg(WIDTH as u32, HEIGHT as u32, "settlement.png")
        // And we call the builder function
        .build()?;

    let mut gs = State {
        ecs: World::new(),
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs.insert(new_map());

    gs.ecs
        .create_entity()
        .with(Position { x: WIDTH / 2, y: HEIGHT / 2 })
        .with(Renderable {
            graphic: TileGraphic::PlayerCharacter,
            layer: TileLayer::PlayerCharacter,
        })
        .with(Player{})
        .build();

    main_loop(context, gs)
}
