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
                        let path = a_star_search(
                            map.to_index(*monster_pos),
                            map.to_index(player_pos),
                            &mut *map,
                        );
                        if path.success && path.steps.len() > 1 {
                            let next_step = map.to_point(path.steps[1]);
                            monster_pos.x = next_step.x;
                            monster_pos.y = next_step.y;
                            monster_viewshed.dirty = true;
                        }
                    }
                }
            }
        }
    }
}
