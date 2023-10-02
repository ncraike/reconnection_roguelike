use super::common::{Menu, MenuDirection, UIAction};
use crate::world::types::{WorldAction, WorldDirection};
use bracket_terminal::prelude::VirtualKeyCode;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Keybound {
    WorldAction(WorldAction),
    UIAction(UIAction),
}
type KeybindingMap = HashMap<VirtualKeyCode, Keybound>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Keybindings {
    pub player_in_world: KeybindingMap,
    pub in_menu: KeybindingMap,
}

pub fn match_key(map: &KeybindingMap, key: Option<VirtualKeyCode>) -> Option<Keybound> {
    match key {
        None => None,
        Some(key) => match map.get(&key) {
            None => None,
            Some(action) => Some(*action),
        },
    }
}

pub fn classic_laptop() -> Keybindings {
    Keybindings {
        player_in_world: HashMap::from([
            // vim-style HJKL cardinal movement
            (
                VirtualKeyCode::H,
                Keybound::WorldAction(WorldAction::Move(WorldDirection::West)),
            ),
            (
                VirtualKeyCode::J,
                Keybound::WorldAction(WorldAction::Move(WorldDirection::South)),
            ),
            (
                VirtualKeyCode::K,
                Keybound::WorldAction(WorldAction::Move(WorldDirection::North)),
            ),
            (
                VirtualKeyCode::L,
                Keybound::WorldAction(WorldAction::Move(WorldDirection::East)),
            ),
            // YUBN diagonal movement
            (
                VirtualKeyCode::Y,
                Keybound::WorldAction(WorldAction::Move(WorldDirection::NorthWest)),
            ),
            (
                VirtualKeyCode::U,
                Keybound::WorldAction(WorldAction::Move(WorldDirection::NorthEast)),
            ),
            (
                VirtualKeyCode::B,
                Keybound::WorldAction(WorldAction::Move(WorldDirection::SouthWest)),
            ),
            (
                VirtualKeyCode::N,
                Keybound::WorldAction(WorldAction::Move(WorldDirection::SouthEast)),
            ),
            (
                VirtualKeyCode::I,
                Keybound::UIAction(UIAction::OpenMenu(Menu::Inventory)),
            ),
            (
                VirtualKeyCode::Escape,
                Keybound::UIAction(UIAction::OpenMenu(Menu::System)),
            ),
            (
                VirtualKeyCode::G,
                Keybound::WorldAction(WorldAction::Pickup),
            ),
            (
                VirtualKeyCode::Period,
                Keybound::WorldAction(WorldAction::Wait),
            ),
        ]),
        in_menu: HashMap::from([
            // Standard confirm/cancel and cardinals
            (
                VirtualKeyCode::Return,
                Keybound::UIAction(UIAction::Confirm),
            ),
            (VirtualKeyCode::Space, Keybound::UIAction(UIAction::Confirm)),
            (VirtualKeyCode::Escape, Keybound::UIAction(UIAction::Cancel)),
            (
                VirtualKeyCode::Left,
                Keybound::UIAction(UIAction::MoveSelection(MenuDirection::Left)),
            ),
            (
                VirtualKeyCode::Down,
                Keybound::UIAction(UIAction::MoveSelection(MenuDirection::Down)),
            ),
            (
                VirtualKeyCode::Up,
                Keybound::UIAction(UIAction::MoveSelection(MenuDirection::Up)),
            ),
            (
                VirtualKeyCode::Right,
                Keybound::UIAction(UIAction::MoveSelection(MenuDirection::Right)),
            ),
            // vim-style HJKL cardinal movement
            (
                VirtualKeyCode::H,
                Keybound::UIAction(UIAction::MoveSelection(MenuDirection::Left)),
            ),
            (
                VirtualKeyCode::J,
                Keybound::UIAction(UIAction::MoveSelection(MenuDirection::Down)),
            ),
            (
                VirtualKeyCode::K,
                Keybound::UIAction(UIAction::MoveSelection(MenuDirection::Up)),
            ),
            (
                VirtualKeyCode::L,
                Keybound::UIAction(UIAction::MoveSelection(MenuDirection::Right)),
            ),
        ]),
    }
}
