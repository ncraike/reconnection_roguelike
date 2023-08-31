use std::collections::HashMap;

use bracket_terminal::prelude::VirtualKeyCode;

#[derive(PartialEq, Eq, Hash)]
pub enum PlayerMoveDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Menu {
    Inventory,
    Character,
    Quests,
    System,
}

#[derive(PartialEq, Eq, Hash)]
pub enum CoreAction {
    MovePlayer(PlayerMoveDirection),
    OpenMenu(Menu),
    Pickup,
    Wait,
}

#[derive(PartialEq, Eq, Hash)]
pub enum MenuDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq, Hash)]
pub enum MenuAction {
    Confirm,
    Cancel,
    MoveSelection(MenuDirection),
}

pub struct Keybindings {
    core: HashMap<VirtualKeyCode, CoreAction>,
    menu: HashMap<VirtualKeyCode, MenuAction>,
}

pub fn classic_laptop() -> Keybindings {
    Keybindings {
        core: HashMap::from([
            // vim-style HJKL cardinal movement
            (
                VirtualKeyCode::H,
                CoreAction::MovePlayer(PlayerMoveDirection::West),
            ),
            (
                VirtualKeyCode::J,
                CoreAction::MovePlayer(PlayerMoveDirection::South),
            ),
            (
                VirtualKeyCode::K,
                CoreAction::MovePlayer(PlayerMoveDirection::North),
            ),
            (
                VirtualKeyCode::L,
                CoreAction::MovePlayer(PlayerMoveDirection::East),
            ),
            // YUBN diagonal movement
            (
                VirtualKeyCode::Y,
                CoreAction::MovePlayer(PlayerMoveDirection::NorthWest),
            ),
            (
                VirtualKeyCode::U,
                CoreAction::MovePlayer(PlayerMoveDirection::NorthEast),
            ),
            (
                VirtualKeyCode::B,
                CoreAction::MovePlayer(PlayerMoveDirection::SouthWest),
            ),
            (
                VirtualKeyCode::N,
                CoreAction::MovePlayer(PlayerMoveDirection::SouthEast),
            ),
            (VirtualKeyCode::I, CoreAction::OpenMenu(Menu::Inventory)),
            (VirtualKeyCode::Escape, CoreAction::OpenMenu(Menu::System)),
            (VirtualKeyCode::G, CoreAction::Pickup),
            (VirtualKeyCode::Period, CoreAction::Wait),
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
