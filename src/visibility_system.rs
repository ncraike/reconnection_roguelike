use bracket_geometry::prelude::Point;
use bracket_pathfinding::prelude::field_of_view;
use specs::prelude::*;

use super::{Map, Player, Position, Viewshed};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed
                .visible_tiles
                .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

            // If this is the player, reveal what they can see
            let p: Option<&Player> = player.get(ent);
            if let Some(_p) = p {
                for vis in viewshed.visible_tiles.iter() {
                    let idx = map.to_index(*vis);
                    map.revealed_terrain[idx] = true;
                }
            }
        }
    }
}
