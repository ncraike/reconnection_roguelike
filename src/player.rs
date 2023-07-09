use bracket_geometry::prelude::Point;
use bracket_terminal::prelude::{BTerm, VirtualKeyCode};
use specs::prelude::*;

use crate::components::{WantsToMelee, WantsToPickupItem};

use super::components::{CombatStats, Item, Player, Viewshed};
use super::map::Map;
use super::message_log::MessageLog;
use super::{InventoryMenuState, Menu, RunState, State};

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
        None => return RunState::AwaitingInput,
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

            // Actions
            VirtualKeyCode::G => get_item(&mut gs.ecs),

            // Menus
            VirtualKeyCode::I => {
                return RunState::ActiveMenu(Menu::Inventory(InventoryMenuState::AwaitingInput))
            }

            // Arrow keys
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),

            _ => return RunState::AwaitingInput,
        },
    }
    RunState::PlayerTurn
}

pub fn player_input_inventory_menu(ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Escape => return RunState::AwaitingInput,
            _ => {}
        },
    }
    RunState::ActiveMenu(Menu::Inventory(InventoryMenuState::AwaitingInput))
}

fn get_item(ecs: &mut World) {
    let entities = ecs.entities();
    let player_entity = ecs.fetch::<Entity>();
    let mut messages = ecs.fetch_mut::<MessageLog>();

    let item_store = ecs.read_storage::<Item>();
    let point_store = ecs.read_storage::<Point>();
    let player_pos = point_store
        .get(*player_entity)
        .expect("Could not get player position");

    let mut target_item: Option<Entity> = None;
    for (item_entity, _item, pos) in (&entities, &item_store, &point_store).join() {
        if pos == player_pos {
            target_item = Some(item_entity);
        }
    }

    match target_item {
        None => messages
            .entries
            .push("There is nothing here to pick up.".to_string()),
        Some(item) => {
            let mut wants_to_pickup_store = ecs.write_storage::<WantsToPickupItem>();
            wants_to_pickup_store
                .insert(
                    *player_entity,
                    WantsToPickupItem {
                        collected_by: *player_entity,
                        item: item,
                    },
                )
                .expect("Unable to add want-to-pickup");
        }
    }
}
