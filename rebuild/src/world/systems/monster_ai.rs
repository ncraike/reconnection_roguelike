use bracket_geometry::prelude::DistanceAlg;
use bracket_pathfinding::prelude::a_star_search;
use specs::prelude::*;
use units::Position2DI32;

use crate::world::units::WorldUnits;

use super::super::super::components::{
    Monster, Player, Viewshed, WantsToMelee, WantsToMove, WorldPosition2D,
};
use super::super::super::map::Map;
use super::super::engine::WorldEngineState;

pub struct MonsterAI {}

impl<'a> MonsterAI {
    fn get_player_pos(
        &self,
        player_store: &ReadStorage<'a, Player>,
        monster_store: &ReadStorage<'a, Monster>,
        position_store: &ReadStorage<'a, WorldPosition2D>,
    ) -> Option<Position2DI32<WorldUnits>> {
        match (player_store, !monster_store, position_store).join().next() {
            None => None,
            Some(player_and_pos) => {
                let (_player, _, player_pos_comp) = player_and_pos;
                Some(player_pos_comp.to_world_units())
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
        ReadStorage<'a, WorldPosition2D>,
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
                for (entity, monster_viewshed, monster_pos_comp, _monster, _) in (
                    &entities,
                    &viewshed_store,
                    &position_store,
                    &monster_store,
                    !&player_store,
                )
                    .join()
                {
                    let monster_pos = monster_pos_comp.to_world_units();
                    if monster_viewshed
                        .visible_tiles
                        .contains(&player_pos.to_bracket_geometry_point())
                    {
                        let distance = DistanceAlg::Pythagoras.distance2d(
                            player_pos.to_bracket_geometry_point(),
                            monster_pos.to_bracket_geometry_point(),
                        );
                        // Player is in melee range: attack
                        if distance < 1.5 {
                            wants_to_melee_store
                                .insert(
                                    entity,
                                    WantsToMelee {
                                        target: *player_entity,
                                    },
                                )
                                .expect("Unable to insert attack against player");
                        // Player is not in melee range: move closer
                        } else {
                            let monster_pos_idx = map.to_index(monster_pos);
                            let player_pos_idx = map.to_index(player_pos);
                            let path = a_star_search(monster_pos_idx, player_pos_idx, &mut *map);
                            if path.success && path.steps.len() > 1 {
                                let destination = map.to_position(path.steps[1]);
                                wants_to_move_store
                                    .insert(
                                        entity,
                                        WantsToMove {
                                            destination: WorldPosition2D::from_world_units(
                                                destination,
                                            ),
                                        },
                                    )
                                    .expect("Queueing monster's move failed");
                            }
                        }
                    }
                }
            }
        }
    }
}
