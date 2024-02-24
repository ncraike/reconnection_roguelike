use bracket_terminal::prelude::{BResult, BTerm, BTermBuilder};
use specs::prelude::*;

use units::{HeightI32, Size2DI32, WidthI32};

use super::keyboard::{match_key, Keybindings, Keybound};
use super::menus::render_inventory_menu;
use super::player_in_world::{player_in_world_controller, render_player_in_world_view};
use super::units::ScreenChars;
use crate::types::{RunState, UITask};
use crate::GAME_TITLE;

pub const TEXT_FONT: &str = "vga8x16.png";
pub const TEXT_FONT_WIDTH: usize = 8;
pub const TEXT_FONT_HEIGHT: usize = 16;
pub const DEFAULT_WINDOW_SIZE: Size2DI32<ScreenChars> = Size2DI32::<ScreenChars> {
    width: WidthI32(ScreenChars(80)),
    height: HeightI32(ScreenChars(25)),
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Consoles {
    WorldTerrain,
    WorldItems,
    WorldActors,
    UIText,
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
            .with_fitscreen(true)
            .with_font(TEXT_FONT, TEXT_FONT_WIDTH, TEXT_FONT_HEIGHT)
            // Terrain
            .with_simple_console(
                DEFAULT_WINDOW_SIZE.width.to_primitive(),
                DEFAULT_WINDOW_SIZE.height.to_primitive(),
                TEXT_FONT,
            )
            // Entities (items)
            .with_sparse_console_no_bg(
                DEFAULT_WINDOW_SIZE.width.to_primitive(),
                DEFAULT_WINDOW_SIZE.height.to_primitive(),
                TEXT_FONT,
            )
            // Entities (player, NPCs, enemies)
            .with_sparse_console_no_bg(
                DEFAULT_WINDOW_SIZE.width.to_primitive(),
                DEFAULT_WINDOW_SIZE.height.to_primitive(),
                TEXT_FONT,
            )
            // Text
            .with_sparse_console_no_bg(
                DEFAULT_WINDOW_SIZE.width.to_primitive(),
                DEFAULT_WINDOW_SIZE.height.to_primitive(),
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
