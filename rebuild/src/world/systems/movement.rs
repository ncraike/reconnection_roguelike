use bracket_terminal::prelude::console;
use specs::prelude::*;

use crate::components::{Name, Viewshed, WantsToMove, WorldPosition2D};
use crate::map::Map;

pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, WorldPosition2D>,
        WriteStorage<'a, WantsToMove>,
        WriteStorage<'a, Viewshed>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, name_store, mut position_store, mut wants_to_move_store, mut viewshed_store) =
            data;

        for (name_component, wants_to_move, position_component, viewshed) in (
            &name_store,
            &wants_to_move_store,
            &mut position_store,
            &mut viewshed_store,
        )
            .join()
        {
            let position = position_component.to_world_units();
            let dest = wants_to_move.destination.to_world_units();
            let dest_idx = map.to_index(dest);
            if !map.bounds().contains(dest) {
                console::log(format!("{} tried to move out of map", name_component.name));
                continue;
            }
            if map.blocked[dest_idx] {
                // FIXME: improve monster turns to avoid collisions
                console::log(format!(
                    "{} could not move to x:{} y:{}, destination blocked",
                    dest.x.to_primitive(),
                    dest.y.to_primitive(),
                    name_component.name
                ));
                continue;
            }
            // Move is possible

            // Copy old position
            let old_position_idx = map.to_index(position.clone());
            // Update position
            position_component.x = dest.x.to_primitive();
            position_component.y = dest.y.to_primitive();
            // Update blocking on map
            map.blocked[dest_idx] = true;
            map.blocked[old_position_idx] = false;
            // Mark entity's vision as needing update
            viewshed.dirty = true;
        }

        wants_to_move_store.clear();
    }
}
