use specs::prelude::{World, WorldExt};

use super::super::types::{RunState, UITask};
use super::systems::damage::delete_the_dead;
use super::systems::run;

#[derive(PartialEq, Copy, Clone)]
pub enum WorldEngineState {
    PlayerTurn,
    MonstersTurn,
}

#[derive(Clone, Copy, Debug)]
pub struct WorldEngine {}

impl WorldEngine {
    pub fn defer_to(&self, world: &mut World) -> RunState {
        let new_run_state: RunState;
        let mut new_engine_state = *world.fetch::<WorldEngineState>();

        match new_engine_state {
            WorldEngineState::PlayerTurn => {
                run(world);
                new_engine_state = WorldEngineState::MonstersTurn;
                new_run_state = RunState::DeferToUIFor(UITask::ShowWorldEvent);
            }
            WorldEngineState::MonstersTurn => {
                run(world);
                new_engine_state = WorldEngineState::PlayerTurn;
                new_run_state = RunState::DeferToUIFor(UITask::GetPlayerAction);
            }
        }

        delete_the_dead(world);

        let mut engine_state_writer = world.write_resource::<WorldEngineState>();
        *engine_state_writer = new_engine_state;

        return new_run_state;
    }
}
