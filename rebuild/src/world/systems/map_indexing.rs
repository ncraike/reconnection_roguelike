use specs::prelude::*;

use super::super::super::components::{BlocksTile, Point};
use super::super::super::map::Map;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Point>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, point_store, blocker_store, entities) = data;

        map.populate_blocked();
        map.clear_content_index();
        for (entity, pt) in (&entities, &point_store).join() {
            let tile_index = map.to_index(*pt);
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
