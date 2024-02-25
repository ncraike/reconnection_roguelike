use bracket_algorithm_traits::prelude::Algorithm2D;
use bracket_pathfinding::prelude::field_of_view;
use specs::prelude::*;

use crate::components::{Player, Viewshed, WorldPosition2D};
use crate::map::Map;
use crate::world::units::WorldUnits;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, WorldPosition2D>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos_store, player) = data;

        for (ent, viewshed, pos_component) in (&entities, &mut viewshed, &pos_store).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(pos_component.to_point(), viewshed.range, &*map);
                viewshed.visible_tiles.retain(|p| map.in_bounds(*p));

                // If this is the player, reveal what they can see
                let _p: Option<&Player> = player.get(ent);
                if let Some(_p) = _p {
                    for t in map.visible_tiles.iter_mut() {
                        *t = false
                    }
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.to_index(WorldUnits::new_position2d_from_point(*vis));
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}
