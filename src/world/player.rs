use bracket_geometry::prelude::Point;
use specs::prelude::*;

use super::super::components::{CombatStats, Item, Player};
use super::super::map::Map;
use super::types::{convert_direction_to_delta, WorldDirection};

pub enum MoveAttemptResult {
    MoveToFreeSpace(Point),
    AttackHostile(Entity),
    // FIXME: add AttackFriendly
    // FIXME: add other interactions, e.g. open door
    // FIXME: add failure case for lack of vision
    Blocked,
}

pub enum PickupAttemptResult {
    ItemAvailable(Entity),
    NothingToPickup,
}

pub fn check_player_move_attempt(world: &World, direction: WorldDirection) -> MoveAttemptResult {
    // FIXME: take a map argument
    let map = world.fetch::<Map>();
    let players_store = world.read_storage::<Player>();
    let position_store = world.read_storage::<Point>();
    let combat_stats = world.read_storage::<CombatStats>();

    for (_player, player_pos) in (&players_store, &position_store).join() {
        let delta = convert_direction_to_delta(direction);
        let dest = *player_pos + delta;

        if !map.bounds().point_in_rect(dest) {
            // Can't move out of bounds (yet)
            // FIXME: add zone transitions?
            return MoveAttemptResult::Blocked;
        }

        let dest_idx = map.to_index(dest);

        if !map.blocked[dest_idx] {
            // Destination is free space
            return MoveAttemptResult::MoveToFreeSpace(dest);
        }

        for potential_target in map.tile_content[dest_idx].iter() {
            // Targets need combat stats to be attacked
            // FIXME: use something else to mark hostility
            let maybe_target = combat_stats.get(*potential_target);
            if let Some(_target) = maybe_target {
                return MoveAttemptResult::AttackHostile(*potential_target);
            }
        }

        // Destination was blocked by something else, e.g. a wall
        return MoveAttemptResult::Blocked;
    }

    // This shouldn't happen, as it means the player has no position and viewshed
    return MoveAttemptResult::Blocked;
}

pub fn check_player_pickup_attempt(world: &World) -> PickupAttemptResult {
    let entities = world.entities();
    let player_entity = world.fetch::<Entity>();

    let item_store = world.read_storage::<Item>();
    let point_store = world.read_storage::<Point>();
    let player_pos = point_store
        .get(*player_entity)
        .expect("Could not get player position");

    for (item_entity, _item, pos) in (&entities, &item_store, &point_store).join() {
        if pos == player_pos {
            return PickupAttemptResult::ItemAvailable(item_entity);
        }
    }
    return PickupAttemptResult::NothingToPickup;
}
