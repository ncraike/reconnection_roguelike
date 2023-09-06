use specs::prelude::*;

use super::super::super::components::{ActionsInWorld, Player};
use super::super::input::keybindings::Keybindings;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        ReadExpect<'a, Keybindings>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, ActionsInWorld>,
    );

    fn run(&mut self, data: Self::SystemData) {}
}
