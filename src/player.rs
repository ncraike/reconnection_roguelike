use bracket_geometry::prelude::Point;
use bracket_terminal::prelude::{BTerm, VirtualKeyCode};
use specs::prelude::*;

use super::components::{Player, Viewshed};
use super::map::{is_passable, Map};
use super::State;

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Point>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, player_pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let dest = *player_pos + Point::new(delta_x, delta_y);
        if !map.bounds().point_in_rect(dest) {
            return;
        }
        let dest_idx = map.to_index(dest);
        if is_passable(map.terrain[dest_idx]) {
            player_pos.x = dest.x;
            player_pos.y = dest.y;
            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
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

            _ => {}
        },
    }
}
