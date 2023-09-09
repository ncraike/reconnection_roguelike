use specs::prelude::World;

use super::super::types::RunState;

pub struct WorldEngine {}

impl WorldEngine {
    pub fn defer_to(world: &mut World) -> RunState {
        RunState::DeferringToUI
    }
}
