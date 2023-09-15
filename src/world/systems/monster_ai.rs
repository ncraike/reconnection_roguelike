use bracket_geometry::prelude::DistanceAlg;
use bracket_pathfinding::prelude::a_star_search;
use specs::prelude::*;

use super::super::super::components::{
    Monster, Player, Point, Viewshed, WantsToMelee, WantsToMove,
};
use super::super::super::map::Map;
use super::super::engine::WorldEngineState;

pub struct MonsterAI {}

impl<'a> MonsterAI {
    fn get_player_pos(
        &self,
        player_store: &ReadStorage<'a, Player>,
        monster_store: &ReadStorage<'a, Monster>,
        point_store: &ReadStorage<'a, Point>,
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
        Entities<'a>,
        WriteExpect<'a, Map>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, WorldEngineState>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Point>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, WantsToMelee>,
        WriteStorage<'a, WantsToMove>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut map,
            player_entity,
            world_engine_state,
            viewshed_store,
            position_store,
            monster_store,
            player_store,
            mut wants_to_melee_store,
            mut wants_to_move_store,
        ) = data;

        if *world_engine_state != WorldEngineState::MonstersTurn {
            return;
        }

        let maybe_player_pos = self.get_player_pos(&player_store, &monster_store, &position_store);
        match maybe_player_pos {
            None => return,
            Some(player_pos) => {
                for (entity, monster_viewshed, monster_pos, _monster, _) in (
                    &entities,
                    &viewshed_store,
                    &position_store,
                    &monster_store,
                    !&player_store,
                )
                    .join()
                {
                    if monster_viewshed.visible_tiles.contains(&player_pos) {
                        let distance = DistanceAlg::Pythagoras.distance2d(player_pos, *monster_pos);
                        if distance < 1.5 {
                            wants_to_melee_store
                                .insert(
                                    entity,
                                    WantsToMelee {
                                        target: *player_entity,
                                    },
                                )
                                .expect("Unable to insert attack against player");
                            return;
                        }
                        let monster_pos_idx = map.to_index(*monster_pos);
                        let player_pos_idx = map.to_index(player_pos);
                        let path = a_star_search(monster_pos_idx, player_pos_idx, &mut *map);
                        if path.success && path.steps.len() > 1 {
                            let destination = map.to_point(path.steps[1]);
                            wants_to_move_store
                                .insert(
                                    entity,
                                    WantsToMove {
                                        destination: destination,
                                    },
                                )
                                .expect("Queueing monster's move failed");
                            return;
                        }
                    }
                }
            }
        }
    }
}
