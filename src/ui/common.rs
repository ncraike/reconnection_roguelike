use bracket_terminal::prelude::{BResult, BTerm, BTermBuilder};
use specs::prelude::*;

use super::super::types::RunState;
use super::super::GAME_TITLE;
use super::keyboard::{match_key, Keybindings, Keybound};
use super::menus::render_inventory_menu;
use super::player_in_world::{player_in_world_controller, render_player_in_world_view};

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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum UIState {
    PlayerInWorld,
    ActiveMenu(Menu),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct NewStates {
    pub ui_state: UIState,
    pub run_state: RunState,
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

        match new_ui_state {
            UIState::PlayerInWorld => {
                render_player_in_world_view(world, ctx);
                let new_states = player_in_world_controller(world, ctx.key);
                new_run_state = new_states.run_state;
                new_ui_state = new_states.ui_state;
            }
            UIState::ActiveMenu(menu) => {
                let keybindings = world.fetch::<Keybindings>();
                match menu {
                    Menu::Inventory => {
                        render_inventory_menu(world, ctx);
                        match match_key(&keybindings.in_menu, ctx.key) {
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
