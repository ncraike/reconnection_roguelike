use bracket_terminal::prelude::{BResult, BTerm, BTermBuilder};
use specs::prelude::*;

use super::super::components::{Player, WantsToMelee, WantsToMove};
use super::super::types::RunState;
use super::super::world::actors::{check_player_move_attempt, MoveAttemptResult, WorldAction};
use super::super::GAME_TITLE;
use super::keyboard::{Keybindings, Keybound};
use super::main_view::render_main_view;
use super::menus::render_inventory_menu;

pub const DEFAULT_WINDOW_WIDTH_IN_TILES: u32 = 48;
pub const DEFAULT_WINDOW_HEIGHT_IN_TILES: u32 = 18;

pub const TILE_1X_WIDTH: u32 = 16;
pub const TILE_1X_HEIGHT: u32 = 24;
pub const TILE_2X_WIDTH: u32 = 32;
pub const TILE_2X_HEIGHT: u32 = 48;
pub const TEXT_FONT_WIDTH: u32 = 8;
pub const TEXT_FONT_HEIGHT: u32 = 16;

pub const DEFAULT_WINDOW_WIDTH_IN_TEXT: u32 =
    DEFAULT_WINDOW_WIDTH_IN_TILES * TILE_2X_WIDTH / TEXT_FONT_WIDTH;
pub const DEFAULT_WINDOW_HEIGHT_IN_TEXT: u32 =
    DEFAULT_WINDOW_HEIGHT_IN_TILES * TILE_2X_HEIGHT / TEXT_FONT_HEIGHT;

pub const TEXT_BOX_HEIGHT: u32 = 6;
pub const TEXT_BOX_HEIGHT_IN_TILES: u32 = TEXT_BOX_HEIGHT * TEXT_FONT_HEIGHT / TILE_2X_HEIGHT;

pub const TILE_1X_FONT: &str = "reconnection_16x24_tiles_at_1x.png";
pub const TILE_2X_FONT: &str = "reconnection_16x24_tiles_at_2x.png";
pub const TEXT_FONT: &str = "vga8x16.png";

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Consoles {
    TilesTerrain,
    TilesEntitiesItems,
    TilesEntitiesCharacters,
    Text,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum UIState {
    PlayerInWorld,
    ActiveMenu(Menu),
}

#[derive(PartialEq, Copy, Clone)]
pub enum InventoryMenuState {
    AwaitingInput,
    UseItem,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum UIAction {
    Confirm,
    Cancel,
    MoveSelection(MenuDirection),
    OpenMenu(Menu),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Menu {
    Inventory,
    Stats,
    Skills,
    System,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum MenuDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug)]
pub struct UI {}

impl UI {
    pub fn build_terminal(&self) -> BResult<BTerm> {
        BTermBuilder::new()
            .with_title(GAME_TITLE)
            .with_tile_dimensions(TILE_2X_WIDTH, TILE_2X_HEIGHT)
            .with_dimensions(
                DEFAULT_WINDOW_WIDTH_IN_TILES,
                DEFAULT_WINDOW_HEIGHT_IN_TILES,
            )
            .with_automatic_console_resize(true)
            .with_fitscreen(true)
            .with_font(TILE_2X_FONT, TILE_2X_WIDTH, TILE_2X_HEIGHT)
            .with_font(TEXT_FONT, TEXT_FONT_WIDTH, TEXT_FONT_HEIGHT)
            // Terrain
            .with_simple_console(
                DEFAULT_WINDOW_WIDTH_IN_TILES,
                DEFAULT_WINDOW_HEIGHT_IN_TILES,
                TILE_2X_FONT,
            )
            // Entities (items)
            .with_sparse_console_no_bg(
                DEFAULT_WINDOW_WIDTH_IN_TILES,
                DEFAULT_WINDOW_HEIGHT_IN_TILES,
                TILE_2X_FONT,
            )
            // Entities (player, NPCs, enemies)
            .with_sparse_console_no_bg(
                DEFAULT_WINDOW_WIDTH_IN_TILES,
                DEFAULT_WINDOW_HEIGHT_IN_TILES,
                TILE_2X_FONT,
            )
            // Text
            .with_sparse_console_no_bg(
                DEFAULT_WINDOW_WIDTH_IN_TEXT,
                DEFAULT_WINDOW_HEIGHT_IN_TEXT,
                TEXT_FONT,
            )
            .build()
    }

    pub fn defer_to(&self, ctx: &mut BTerm, world: &mut World) -> RunState {
        let mut new_run_state: RunState = RunState::DeferringToUI;
        let mut new_ui_state = *world.fetch::<UIState>();

        let entities = world.entities();
        let player_store = world.read_storage::<Player>();
        let mut wants_to_move_store = world.write_storage::<WantsToMove>();
        let mut wants_to_melee_store = world.write_storage::<WantsToMelee>();

        let keybindings = world.fetch::<Keybindings>();

        match new_ui_state {
            UIState::PlayerInWorld => {
                render_main_view(world, ctx);
                match keybindings.world_focus_action(ctx.key.clone()) {
                    None => (),
                    Some(keybound) => {
                        match keybound {
                            Keybound::WorldAction(world_action) => {
                                match world_action {
                                    WorldAction::Move(direction) => {
                                        let move_attempt_result =
                                            check_player_move_attempt(world, direction);
                                        match move_attempt_result {
                                            MoveAttemptResult::MoveToFreeSpace(destination) => {
                                                for (player_entity, _player_component) in
                                                    (&entities, &player_store).join()
                                                {
                                                    wants_to_move_store
                                                        .insert(
                                                            player_entity,
                                                            WantsToMove {
                                                                destination: destination,
                                                            },
                                                        )
                                                        .expect("Queueing player move failed");
                                                }
                                            }
                                            MoveAttemptResult::AttackHostile(target) => {
                                                for (player_entity, _player_component) in
                                                    (&entities, &player_store).join()
                                                {
                                                    wants_to_melee_store
                                                        .insert(
                                                            player_entity,
                                                            WantsToMelee { target: target },
                                                        )
                                                        .expect(
                                                            "Queueing player melee attack failed",
                                                        );
                                                }
                                            }
                                            // FIXME: give some UI feedback
                                            MoveAttemptResult::Blocked => (),
                                        }
                                    }
                                    // FIXME
                                    WorldAction::Pickup => (),
                                    WorldAction::Wait => (),
                                }
                                //queue_world_action(world_action);
                                new_run_state = RunState::WorldTick;
                            }
                            Keybound::UIAction(ui_action) => match ui_action {
                                UIAction::OpenMenu(menu) => {
                                    new_ui_state = UIState::ActiveMenu(menu);
                                }
                                _ => (),
                            },
                        }
                    }
                }
            }
            UIState::ActiveMenu(menu) => {
                match menu {
                    Menu::Inventory => {
                        render_inventory_menu(world, ctx);
                        match keybindings.menu_focus_action(ctx.key.clone()) {
                            None => (),
                            Some(keybound) => match keybound {
                                Keybound::WorldAction(_) => (),
                                Keybound::UIAction(ui_action) => match ui_action {
                                    UIAction::Cancel => {
                                        new_ui_state = UIState::PlayerInWorld;
                                    }
                                    _ => (),
                                },
                            },
                        }
                    }
                    // FIXME: add other menus
                    _ => (),
                }
            }
        }

        let mut ui_state_writer = world.write_resource::<UIState>();
        *ui_state_writer = new_ui_state;

        return new_run_state;
    }
}
