pub mod components;
pub mod map;
pub mod message_log;
pub mod types;
pub mod ui;
pub mod world;

use bracket_lib::prelude::GameState;
use bracket_terminal;
use bracket_terminal::prelude::BTerm;

use specs::prelude::*;

use types::{RunState, UITask};
use ui::UI;
use world::systems;
use world::WorldEngine; // FIXME: move to WorldEngine setup

pub const GAME_TITLE: &str = "Reconnection";

pub struct State {
    pub ecs: World,
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
