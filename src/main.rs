use bracket_terminal::prelude::*;

use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;

bracket_terminal::embedded_resource!(TILE_FONT, "../resources/settlement.png");

pub struct State {
    ecs: World,
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
