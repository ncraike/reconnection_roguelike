use bracket_geometry::prelude::Point;
use bracket_lib::prelude::{main_loop, GameState};
use bracket_terminal;
use bracket_terminal::prelude::{BError, BTerm, EMBED};

use specs::prelude::*;

pub mod components;
pub mod map;
pub mod message_log;
pub mod types;
pub mod ui;
pub mod world;

use components::register_components;
use map::{Map, MAP_HEIGHT, MAP_WIDTH};
use message_log::MessageLog;
use types::{RunState, UITask};
use ui::common::UIState; // FIXME: move to UI setup
use ui::keyboard::{classic_laptop, Keybindings};
use ui::UI;
use world::engine::WorldEngineState;
use world::spawner::{
    create_bandage, create_enemy_big_stalker, create_enemy_hound, create_first_aid_kit,
    create_player,
};
use world::systems;
use world::WorldEngine; // FIXME: move to WorldEngine setup

pub const GAME_TITLE: &str = "Reconnection";

bracket_terminal::embedded_resource!(TILE_FONT, "../resources/reconnection_16x24_tiles_at_2x.png");

pub struct State {
    ecs: World,
}

// Implement the game loop
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let ui = *self.ecs.fetch::<UI>();
        let world_engine = *self.ecs.fetch::<WorldEngine>();

        let mut new_run_state;
        {
            let run_state = self.ecs.fetch::<RunState>();
            new_run_state = *run_state;
        }

        match new_run_state {
            RunState::PreRun => {
                self.run_systems();
                new_run_state = RunState::DeferToUIFor(UITask::GetPlayerAction);
            }
            RunState::DeferToUIFor(ui_task) => match ui_task {
                UITask::GetPlayerAction => {
                    new_run_state = ui.defer_to_get_player_action(ctx, &mut self.ecs);
                }
                UITask::ShowWorldEvent => {
                    new_run_state = ui.defer_to_show_world_event(ctx, &mut self.ecs);
                }
            },
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

    let mut gs = State { ecs: World::new() };
    register_components(&mut gs.ecs);

    let ui: UI = UI {};
    gs.ecs.insert::<UI>(UI {});
    gs.ecs.insert::<UIState>(UIState::PlayerInWorld); // FIXME: move to UI setup
    gs.ecs.insert::<Keybindings>(classic_laptop()); // FIXME: move to UI setup

    let ctx = ui.build_terminal()?;

    let world_engine: WorldEngine = WorldEngine {};
    gs.ecs.insert::<WorldEngine>(world_engine);
    gs.ecs
        .insert::<WorldEngineState>(WorldEngineState::PlayerTurn); // FIXME: move to WorldEngine setup

    gs.ecs.insert(RunState::PreRun);

    let map: Map = Map::new_map();
    gs.ecs.insert(map);

    gs.ecs.insert(MessageLog { entries: vec![] });

    let player = create_player(
        &mut gs.ecs,
        Point {
            x: (MAP_WIDTH / 2) as i32,
            y: (MAP_HEIGHT / 2) as i32,
        },
    );
    gs.ecs.insert(player);
    create_enemy_hound(
        &mut gs.ecs,
        Point {
            x: (MAP_WIDTH / 2 + MAP_WIDTH / 4) as i32,
            y: (MAP_HEIGHT / 4) as i32,
        },
    );
    create_enemy_big_stalker(
        &mut gs.ecs,
        Point {
            x: (MAP_WIDTH / 2 + MAP_WIDTH / 4) as i32,
            y: (MAP_HEIGHT / 2 + MAP_HEIGHT / 4) as i32,
        },
    );
    create_bandage(
        &mut gs.ecs,
        Point {
            x: (MAP_WIDTH / 2 - MAP_WIDTH / 4) as i32,
            y: (MAP_HEIGHT / 2 + MAP_HEIGHT / 4) as i32,
        },
    );
    create_bandage(
        &mut gs.ecs,
        Point {
            x: ((MAP_WIDTH / 2 - MAP_WIDTH / 4) + 1) as i32,
            y: (MAP_HEIGHT / 2 + MAP_HEIGHT / 4) as i32,
        },
    );
    create_first_aid_kit(
        &mut gs.ecs,
        Point {
            x: (MAP_WIDTH / 2 - MAP_WIDTH / 4) as i32,
            y: ((MAP_HEIGHT / 2 + MAP_HEIGHT / 4) + 1) as i32,
        },
    );

    main_loop(ctx, gs)
}
