use bracket_color::prelude::{ColorPair, RGB};
use bracket_geometry::prelude::Point;
use bracket_lib::prelude::{main_loop, GameState};
use bracket_terminal;
use bracket_terminal::prelude::{
    render_draw_buffer, BError, BTerm, BTermBuilder, DrawBatch, EMBED,
};

use specs::prelude::*;

pub mod components;
use components::{Player, Position, Renderable};
pub mod map;
use map::{draw_map, Map, TileGraphic, HEIGHT, TILE_HEIGHT, TILE_WIDTH, WIDTH};
mod player;
use player::player_input;

bracket_terminal::embedded_resource!(TILE_FONT, "../resources/settlement.png");

pub struct State {
    ecs: World,
}

// Implement the game loop
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs);
        render_draw_buffer(ctx).expect("Render error");

        let mut draw_batch = DrawBatch::new();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        draw_batch.target(1);
        draw_batch.cls();
        for (pos, render) in (&positions, &renderables).join() {
            draw_batch.set(
                Point { x: pos.x, y: pos.y },
                ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0., 0., 0.)),
                render.graphic as u16,
            );
            draw_batch.submit(0).expect("Batch error");
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
        .with_dimensions(WIDTH, HEIGHT)
        .with_tile_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .with_title("Reconnection - Settlement")
        .with_font("reconnection_16x24.png", TILE_WIDTH, TILE_HEIGHT)
        .with_simple_console(WIDTH as u32, HEIGHT as u32, "reconnection_16x24.png")
        .with_sparse_console_no_bg(WIDTH as u32, HEIGHT as u32, "reconnection_16x24.png")
        .build()?;

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let map: Map = Map::new_map();
    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position {
            x: WIDTH / 2,
            y: HEIGHT / 2,
        })
        .with(Renderable {
            graphic: TileGraphic::PlayerCharacter,
        })
        .with(Player {})
        .build();

    main_loop(context, gs)
}
