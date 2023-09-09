use bracket_terminal::prelude::VirtualKeyCode;
use specs::prelude::*;

use super::super::super::components::{ActionsInWorld, Player};
use super::super::keyboard::Keybindings;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        ReadExpect<'a, Option<VirtualKeyCode>>,
        ReadExpect<'a, Keybindings>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, ActionsInWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (maybe_key, keybindings, player_store, mut actions_store) = data;
        match keybindings.get_core_action(*maybe_key) {
            None => (),
            Some(action) => {
                for (_player, actions) in (&player_store, &mut actions_store).join() {
                    actions.actions.push(action);
                }
            }
        }
    }
}
