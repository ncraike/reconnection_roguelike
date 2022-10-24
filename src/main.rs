use bracket_lib::prelude::{main_loop, GameState};
use bracket_terminal;
use bracket_terminal::prelude::{BError, BTerm, BTermBuilder, EMBED};

use specs::prelude::*;

pub mod camera;
pub mod components;
pub mod map;
pub mod player;
pub mod visibility_system;
use camera::{render_camera, DEFAULT_VIEW_HEIGHT, DEFAULT_VIEW_WIDTH};
use components::{Player, Point, Renderable, Viewshed};
use map::{Map, TileGraphic, MAP_HEIGHT, MAP_WIDTH, TILE_2X_HEIGHT, TILE_2X_WIDTH};
use player::player_input;
use visibility_system::VisibilitySystem;

bracket_terminal::embedded_resource!(TILE_FONT, "../resources/reconnection_16x24_tiles_at_2x.png");

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
    bracket_terminal::link_resource!(TILE_FONT, "../resources/reconnection_16x24_tiles_at_2x.png");

    let context = BTermBuilder::new()
        .with_dimensions(DEFAULT_VIEW_WIDTH, DEFAULT_VIEW_HEIGHT)
        .with_tile_dimensions(TILE_2X_WIDTH, TILE_2X_HEIGHT)
        .with_title("Reconnection")
        .with_font(
            "reconnection_16x24_tiles_at_2x.png",
            TILE_2X_WIDTH,
            TILE_2X_HEIGHT,
        )
        .with_simple_console(
            DEFAULT_VIEW_WIDTH,
            DEFAULT_VIEW_HEIGHT,
            "reconnection_16x24_tiles_at_2x.png",
        )
        .with_sparse_console_no_bg(
            DEFAULT_VIEW_WIDTH,
            DEFAULT_VIEW_HEIGHT,
            "reconnection_16x24_tiles_at_2x.png",
        )
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
            x: (MAP_WIDTH / 2) as i32,
            y: (MAP_HEIGHT / 2) as i32,
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

    gs.ecs
        .create_entity()
        .with(Point {
            x: (MAP_WIDTH / 2 + MAP_WIDTH / 4) as i32,
            y: (MAP_HEIGHT / 4) as i32,
        })
        .with(Renderable {
            graphic: TileGraphic::EnemyHound,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();
    gs.ecs
        .create_entity()
        .with(Point {
            x: (MAP_WIDTH / 2 + MAP_WIDTH / 4) as i32,
            y: (MAP_HEIGHT / 2 + MAP_HEIGHT / 4) as i32,
        })
        .with(Renderable {
            graphic: TileGraphic::EnemyBigStalker,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    main_loop(context, gs)
}
