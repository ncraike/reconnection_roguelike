use bracket_geometry::prelude::Point;
use bracket_terminal::prelude::{BTerm, VirtualKeyCode};
use specs::prelude::*;

use crate::components::WantsToMelee;

use super::components::{CombatStats, Player, Viewshed};
use super::map::Map;
use super::{RunState, State};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Point>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut wants_to_melee_store = ecs.write_storage::<WantsToMelee>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let entities = ecs.entities();
    let map = ecs.fetch::<Map>();

    for (entity, _player, player_pos, viewshed) in
        (&entities, &mut players, &mut positions, &mut viewsheds).join()
    {
        let dest = *player_pos + Point::new(delta_x, delta_y);
        if !map.bounds().point_in_rect(dest) {
            return;
        }
        let dest_idx = map.to_index(dest);
        for potential_target in map.tile_content[dest_idx].iter() {
            let maybe_target = combat_stats.get(*potential_target);
            if let Some(_target) = maybe_target {
                wants_to_melee_store
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *potential_target,
                        },
                    )
                    .expect("Add melee target failed");
            }
        }

        if !map.blocked[dest_idx] {
            player_pos.x = dest.x;
            player_pos.y = dest.y;
            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    // Player movement
    match ctx.key {
        None => return RunState::Paused,
        Some(key) => match key {
            // Laptop controls

            // vim-style HJKL
            VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),
            // diagonals on YUBN
            VirtualKeyCode::Y => try_move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::U => try_move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),
            VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),

            // Arrow keys
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),

            _ => return RunState::Paused,
        },
    }
    RunState::Running
}
