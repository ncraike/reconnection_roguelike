use bracket_lib::prelude::{main_loop, GameState};
use bracket_terminal;
use bracket_terminal::prelude::{BError, BTerm, EMBED};

use specs::prelude::*;

pub mod components;
pub mod map;
pub mod message_log;
pub mod player;
pub mod types;
pub mod ui;
pub mod world;

use components::register_components;
use map::Map;
use message_log::MessageLog;
use player::{do_player_action, player_input_inventory_menu};
use types::RunState;
use ui::{BTermTiledUI, UI};
use world::spawner::default_spawn;
use world::systems;
use world::systems::damage::delete_the_dead;
use world::systems::map_indexing::MapIndexingSystem;
use world::systems::visibility::VisibilitySystem;
use world::WorldEngine;

pub const GAME_TITLE: &str = "Reconnection";

bracket_terminal::embedded_resource!(TILE_FONT, "../resources/reconnection_16x24_tiles_at_2x.png");

#[derive(PartialEq, Copy, Clone)]
pub enum InventoryMenuState {
    AwaitingInput,
    UseItem,
}

pub struct State {
    ecs: World,
}

// Implement the game loop
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let ui = self.ecs.fetch::<UI>();
        let world_engine = self.ecs.fetch::<WorldEngine>();

        let mut new_run_state;
        {
            let run_state = self.ecs.fetch::<RunState>();
            new_run_state = *run_state;
        }

        match new_run_state {
            RunState::PreRun => {
                self.run_systems();
                new_run_state = RunState::AwaitingInput;
            }
            RunState::DeferringToUI => {
                new_run_state = ui.defer_to(ctx, &self.ecs);
            }
            RunState::WorldTick => {
                new_run_state = world_engine.defer_to(&mut self.ecs);
            }
        }

        {
            let mut run_state_writer = self.ecs.write_resource::<RunState>();
            *run_state_writer = new_run_state;
        }

        self.ecs.maintain();
    }
}

impl State {
    fn run_systems(&mut self) {
        systems::run(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> BError {
    bracket_terminal::link_resource!(TILE_FONT, "../resources/reconnection_16x24_tiles_at_2x.png");

    let ui: UI = BTermTiledUI {};
    let ui_context: UI::Context = ui.build_context();

    let world_engine = WorldEngine {};

    let mut gs = State { ecs: World::new() };
    register_components(&mut gs.ecs);

    gs.ecs.insert::<UI>(ui);
    gs.ecs.insert(world_engine);
    gs.ecs.insert(RunState::PreRun);

    let map: Map = Map::new_map();
    gs.ecs.insert(map);

    gs.ecs.insert(MessageLog { entries: vec![] });

    default_spawn(&mut gs.ecs);

    main_loop(ui_context, gs)
}
