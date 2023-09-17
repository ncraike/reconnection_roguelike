use bracket_geometry::prelude::Point;
use specs::prelude::*;

use super::super::super::components::{Viewshed, WantsToMove};
use super::super::super::map::Map;
use super::super::super::message_log::MessageLog;

pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteExpect<'a, MessageLog>,
        WriteStorage<'a, Point>,
        WriteStorage<'a, WantsToMove>,
        WriteStorage<'a, Viewshed>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, mut log, mut position_store, mut wants_to_move_store, mut viewshed_store) =
            data;

        for (wants_to_move, mut position, mut viewshed) in (
            &wants_to_move_store,
            &mut position_store,
            &mut viewshed_store,
        )
            .join()
        {
            if !map.bounds().point_in_rect(wants_to_move.destination) {
                log.entries.push("Could not move out of bounds".to_string());
                continue;
            }
            let dest_idx = map.to_index(wants_to_move.destination);
            if map.blocked[dest_idx] {
                log.entries
                    .push("Could not move; destination blocked".to_string());
                continue;
            }

            // Move is possible

            let old_position = wants_to_move.destination.clone();
            let old_position_idx = map.to_index(old_position);

            // Update position
            position.x = wants_to_move.destination.x;
            position.y = wants_to_move.destination.y;
            // Update blocking on map
            map.blocked[dest_idx] = true;
            map.blocked[old_position_idx] = false;
            // Mark entity's viewshed as needing update
            viewshed.dirty = true;
        }

        wants_to_move_store.clear();
    }
}
