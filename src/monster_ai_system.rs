use bracket_geometry::prelude::DistanceAlg;
use bracket_pathfinding::prelude::a_star_search;
use bracket_terminal::prelude::console;
use specs::prelude::*;

use super::{Map, Monster, Name, Player, Point, Viewshed};

pub struct MonsterAI {}

impl<'a> MonsterAI {
    fn get_player_pos(
        &self,
        player_store: &ReadStorage<'a, Player>,
        monster_store: &ReadStorage<'a, Monster>,
        point_store: &WriteStorage<'a, Point>,
    ) -> Option<Point> {
        match (player_store, !monster_store, point_store).join().next() {
            None => None,
            Some(player_and_pos) => {
                let (_player, _, player_pos) = player_and_pos;
                Some(*player_pos)
            }
        }
    }
}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Point>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, mut viewshed_store, mut point_store, monster_store, name_store, player_store) =
            data;

        let maybe_player_pos = self.get_player_pos(&player_store, &monster_store, &point_store);
        match maybe_player_pos {
            None => return,
            Some(player_pos) => {
                for (mut monster_viewshed, mut monster_pos, _monster, monster_name, _) in (
                    &mut viewshed_store,
                    &mut point_store,
                    &monster_store,
                    &name_store,
                    !&player_store,
                )
                    .join()
                {
                    if monster_viewshed.visible_tiles.contains(&player_pos) {
                        let distance = DistanceAlg::Pythagoras.distance2d(player_pos, *monster_pos);
                        if distance < 1.5 {
                            // Attack goes here
                            console::log(&format!("{} beeps aggressively", monster_name.name));
                            return;
                        }
                        let monster_pos_idx = map.to_index(*monster_pos);
                        let player_pos_idx = map.to_index(player_pos);
                        let path = a_star_search(monster_pos_idx, player_pos_idx, &mut *map);
                        if path.success && path.steps.len() > 1 {
                            let new_pos_idx = path.steps[1];
                            let new_pos = map.to_point(new_pos_idx);
                            monster_pos.x = new_pos.x;
                            monster_pos.y = new_pos.y;
                            // Mark new position as blocked, old position as unblocked
                            // FIXME: it'd be kinda nice if map_index_system handled this
                            map.blocked[new_pos_idx] = true;
                            map.blocked[monster_pos_idx] = false;
                            // Mark visibility for update
                            monster_viewshed.dirty = true;
                        }
                    }
                }
            }
        }
    }
}
