use bracket_geometry::prelude::Point;
use bracket_terminal::prelude::console;
use specs::prelude::*;

use super::super::super::components::{Name, Viewshed, WantsToMove};
use super::super::super::map::Map;

pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Point>,
        WriteStorage<'a, WantsToMove>,
        WriteStorage<'a, Viewshed>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, name_store, mut position_store, mut wants_to_move_store, mut viewshed_store) =
            data;

        for (name_component, wants_to_move, position, viewshed) in (
            &name_store,
            &wants_to_move_store,
            &mut position_store,
            &mut viewshed_store,
        )
            .join()
        {
            let dest = wants_to_move.destination.clone();
            let dest_idx = map.to_index(dest);
            if !map.bounds().point_in_rect(dest) {
                console::log(format!("{} tried to move out of map", name_component.name));
                continue;
            }
            if map.blocked[dest_idx] {
                // FIXME: improve monster turns to avoid collisions
                console::log(format!(
                    "{} could not move to x:{} y:{}, destination blocked",
                    dest.x, dest.y, name_component.name
                ));
                continue;
            }
            // Move is possible

            // Copy old position
            let old_position = position.clone();
            let old_position_idx = map.to_index(old_position);
            // Update position
            position.x = dest.x;
            position.y = dest.y;
            // Update blocking on map
            map.blocked[dest_idx] = true;
            map.blocked[old_position_idx] = false;
            // Mark entity's vision as needing update
            viewshed.dirty = true;
        }

        wants_to_move_store.clear();
    }
}
