use bracket_lib::prelude::{main_loop, GameState};
use bracket_terminal;
use bracket_terminal::prelude::{BError, BTerm, BTermBuilder, EMBED};

use specs::prelude::*;

pub mod camera;
pub mod components;
pub mod map;
pub mod player;
pub mod visibility_system;
use camera::render_camera;
use components::{Player, Point, Renderable, Viewshed};
use map::{Map, TileGraphic, HEIGHT, TILE_HEIGHT, TILE_WIDTH, WIDTH};
use player::player_input;
use visibility_system::VisibilitySystem;

bracket_terminal::embedded_resource!(TILE_FONT, "../resources/settlement.png");

pub struct State {
    ecs: World,
}

// Implement the game loop
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        player_input(self, ctx);
        self.run_systems();

        render_camera(&self.ecs, ctx);
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> BError {
    bracket_terminal::link_resource!(TILE_FONT, "resources/settlement.png");

    let context = BTermBuilder::new()
        .with_dimensions(WIDTH, HEIGHT)
        .with_tile_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .with_title("Reconnection - Settlement")
        .with_font("reconnection_16x24.png", TILE_WIDTH, TILE_HEIGHT)
        .with_simple_console(WIDTH as u32, HEIGHT as u32, "reconnection_16x24.png")
        .with_sparse_console_no_bg(WIDTH as u32, HEIGHT as u32, "reconnection_16x24.png")
        .build()?;

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Point>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map: Map = Map::new_map();
    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Point {
            x: WIDTH / 2,
            y: HEIGHT / 2,
        })
        .with(Renderable {
            graphic: TileGraphic::PlayerCharacter,
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    main_loop(context, gs)
}
