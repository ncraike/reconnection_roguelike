// use bracket_geometry::prelude::Point;
// use bracket_pathfinding::prelude::field_of_view;
use bracket_terminal::prelude::console;
use specs::prelude::*;

use super::{Monster, Name, Player, Point, Viewshed};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Point>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (viewshed_store, point_store, monster_store, name_store, player_store) = data;
        match (&player_store, &point_store).join().next() {
            None => return,
            Some(player_and_pos) => {
                let (_player, player_pos) = player_and_pos;

                for (monster_viewshed, _monster_pos, _monster, monster_name) in
                    (&viewshed_store, &point_store, &monster_store, &name_store).join()
                {
                    if monster_viewshed.visible_tiles.contains(&player_pos) {
                        console::log(&format!("{} beeps aggressively", monster_name.name));
                    }
                }
            }
        };
    }
}
