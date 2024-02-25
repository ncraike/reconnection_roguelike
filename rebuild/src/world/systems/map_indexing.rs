use specs::prelude::*;

use crate::components::{BlocksTile, WorldPosition2D};
use crate::map::Map;
use crate::world::units::WorldUnits;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, WorldPosition2D>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, point_store, blocker_store, entities) = data;

        map.populate_blocked();
        map.clear_content_index();
        for (entity, pt_component) in (&entities, &point_store).join() {
            let tile_index = map.to_index(WorldUnits::new_position2d_from_component(*pt_component));
            // If entity blocks, update blocking list
            let _p: Option<&BlocksTile> = blocker_store.get(entity);
            if let Some(_p) = _p {
                map.blocked[tile_index] = true;
            }
            // Add entity to this tile's contents
            map.tile_content[tile_index].push(entity);
        }
    }
}
