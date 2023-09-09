use std::collections::HashMap;

use bracket_terminal::prelude::{BTerm, VirtualKeyCode};

use super::super::player::{Menu, PlayerAction, PlayerMoveDirection};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum MenuDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum MenuAction {
    Confirm,
    Cancel,
    MoveSelection(MenuDirection),
}

pub struct Keybindings {
    core: HashMap<VirtualKeyCode, PlayerAction>,
    menu: HashMap<VirtualKeyCode, MenuAction>,
}

impl Keybindings {
    fn match_key<Action: Copy>(
        map: &HashMap<VirtualKeyCode, Action>,
        key: Option<VirtualKeyCode>,
    ) -> Option<Action> {
        match key {
            None => None,
            Some(key) => match map.get(&key) {
                None => None,
                Some(action) => Some(*action),
            },
        }
    }

    pub fn get_core_action(&self, key: Option<VirtualKeyCode>) -> Option<PlayerAction> {
        Keybindings::match_key(&self.core, key)
    }

    pub fn get_menu_action(&self, key: Option<VirtualKeyCode>) -> Option<MenuAction> {
        Keybindings::match_key(&self.menu, key)
    }
}

pub fn classic_laptop() -> Keybindings {
    Keybindings {
        core: HashMap::from([
            // vim-style HJKL cardinal movement
            (
                VirtualKeyCode::H,
                PlayerAction::MovePlayer(PlayerMoveDirection::West),
            ),
            (
                VirtualKeyCode::J,
                PlayerAction::MovePlayer(PlayerMoveDirection::South),
            ),
            (
                VirtualKeyCode::K,
                PlayerAction::MovePlayer(PlayerMoveDirection::North),
            ),
            (
                VirtualKeyCode::L,
                PlayerAction::MovePlayer(PlayerMoveDirection::East),
            ),
            // YUBN diagonal movement
            (
                VirtualKeyCode::Y,
                PlayerAction::MovePlayer(PlayerMoveDirection::NorthWest),
            ),
            (
                VirtualKeyCode::U,
                PlayerAction::MovePlayer(PlayerMoveDirection::NorthEast),
            ),
            (
                VirtualKeyCode::B,
                PlayerAction::MovePlayer(PlayerMoveDirection::SouthWest),
            ),
            (
                VirtualKeyCode::N,
                PlayerAction::MovePlayer(PlayerMoveDirection::SouthEast),
            ),
            (VirtualKeyCode::I, PlayerAction::OpenMenu(Menu::Inventory)),
            (VirtualKeyCode::Escape, PlayerAction::OpenMenu(Menu::System)),
            (VirtualKeyCode::G, PlayerAction::Pickup),
            (VirtualKeyCode::Period, PlayerAction::Wait),
        ]),
        menu: HashMap::from([
            // Standard confirm/cancel and cardinals
            (VirtualKeyCode::Return, MenuAction::Confirm),
            (VirtualKeyCode::Space, MenuAction::Confirm),
            (VirtualKeyCode::Escape, MenuAction::Cancel),
            (
                VirtualKeyCode::Up,
                MenuAction::MoveSelection(MenuDirection::Up),
            ),
            (
                VirtualKeyCode::Left,
                MenuAction::MoveSelection(MenuDirection::Left),
            ),
            (
                VirtualKeyCode::Down,
                MenuAction::MoveSelection(MenuDirection::Down),
            ),
            (
                VirtualKeyCode::Right,
                MenuAction::MoveSelection(MenuDirection::Right),
            ),
            // vim-style HJKL cardinal movement
            (
                VirtualKeyCode::H,
                MenuAction::MoveSelection(MenuDirection::Left),
            ),
            (
                VirtualKeyCode::J,
                MenuAction::MoveSelection(MenuDirection::Down),
            ),
            (
                VirtualKeyCode::K,
                MenuAction::MoveSelection(MenuDirection::Up),
            ),
            (
                VirtualKeyCode::L,
                MenuAction::MoveSelection(MenuDirection::Right),
            ),
        ]),
    }
}
