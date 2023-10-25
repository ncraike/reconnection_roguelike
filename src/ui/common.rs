use bracket_terminal::prelude::{BResult, BTerm, BTermBuilder};
use specs::prelude::*;

use crate::ui::units::{Height, Point2D, Size2D, Tiles2x, Width, ONE_TEXT_CHAR, ONE_TILE2X};

use super::super::types::{RunState, UITask};
use super::super::GAME_TITLE;
use super::keyboard::{match_key, Keybindings, Keybound};
use super::menus::render_inventory_menu;
use super::player_in_world::{player_in_world_controller, render_player_in_world_view};

pub const DEFAULT_WINDOW_SIZE: Size2D<Tiles2x> = Size2D::<Tiles2x> {
    width: Width(Tiles2x(48)),
    height: Height(Tiles2x(18)),
};

// pub const TILE_1X_WIDTH: u32 = 16;
// pub const TILE_1X_HEIGHT: u32 = 24;
// pub const TILE_2X_WIDTH: u32 = 32;
// pub const TILE_2X_HEIGHT: u32 = 48;
// pub const TEXT_FONT_WIDTH: u32 = 8;
// pub const TEXT_FONT_HEIGHT: u32 = 16;

// pub const DEFAULT_WINDOW_WIDTH_IN_TEXT: u32 =
//     DEFAULT_WINDOW_WIDTH_IN_TILES * TILE_2X_WIDTH / TEXT_FONT_WIDTH;
// pub const DEFAULT_WINDOW_HEIGHT_IN_TEXT: u32 =
//     DEFAULT_WINDOW_HEIGHT_IN_TILES * TILE_2X_HEIGHT / TEXT_FONT_HEIGHT;

// pub const TEXT_BOX_HEIGHT: u32 = 6;
// pub const TEXT_BOX_HEIGHT_IN_TILES: u32 = TEXT_BOX_HEIGHT * TEXT_FONT_HEIGHT / TILE_2X_HEIGHT;

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
            .with_tile_dimensions(
                ONE_TILE2X.width.to_pixels().to_primitive(),
                ONE_TILE2X.height.to_pixels().to_primitive(),
            )
            .with_dimensions(
                DEFAULT_WINDOW_SIZE.width.to_primitive(),
                DEFAULT_WINDOW_SIZE.height.to_primitive(),
            )
            .with_automatic_console_resize(true)
            .with_fitscreen(true)
            .with_font(
                TILE_2X_FONT,
                ONE_TILE2X.width.to_pixels().to_primitive(),
                ONE_TILE2X.height.to_pixels().to_primitive(),
            )
            .with_font(
                TEXT_FONT,
                ONE_TEXT_CHAR.width.to_pixels().to_primitive(),
                ONE_TEXT_CHAR.height.to_pixels().to_primitive(),
            )
            // Terrain
            .with_simple_console(
                DEFAULT_WINDOW_SIZE.width.to_primitive(),
                DEFAULT_WINDOW_SIZE.height.to_primitive(),
                TILE_2X_FONT,
            )
            // Entities (items)
            .with_sparse_console_no_bg(
                DEFAULT_WINDOW_SIZE.width.to_primitive(),
                DEFAULT_WINDOW_SIZE.height.to_primitive(),
                TILE_2X_FONT,
            )
            // Entities (player, NPCs, enemies)
            .with_sparse_console_no_bg(
                DEFAULT_WINDOW_SIZE.width.to_primitive(),
                DEFAULT_WINDOW_SIZE.height.to_primitive(),
                TILE_2X_FONT,
            )
            // Text
            .with_sparse_console_no_bg(
                DEFAULT_WINDOW_SIZE
                    .to_text_chars_floor()
                    .width
                    .to_primitive(),
                DEFAULT_WINDOW_SIZE
                    .to_text_chars_floor()
                    .height
                    .to_primitive(),
                TEXT_FONT,
            )
            .build()
    }

    pub fn defer_to_get_player_action(&self, ctx: &mut BTerm, world: &mut World) -> RunState {
        let mut new_run_state: RunState = RunState::DeferToUIFor(UITask::GetPlayerAction);
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

    pub fn defer_to_show_world_event(&self, ctx: &mut BTerm, world: &mut World) -> RunState {
        render_player_in_world_view(world, ctx);
        return RunState::WorldTick;
    }
}
