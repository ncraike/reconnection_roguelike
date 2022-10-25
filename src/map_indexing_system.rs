use super::{BlocksTile, Map, Point};
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Point>,
        ReadStorage<'a, BlocksTile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, point_store, blocker_store) = data;

        map.populate_blocked();
        for (pos, _blocks) in (&point_store, &blocker_store).join() {
            let tile_index = map.to_index(*pos);
            map.blocked[tile_index] = true;
        }
    }
}
