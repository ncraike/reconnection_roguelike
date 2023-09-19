use bracket_terminal::prelude::VirtualKeyCode;
use specs::prelude::*;

use crate::components::{Player, WantsToMelee, WantsToMove};
use crate::types::RunState;
use crate::ui::common::{NewStates, UIAction, UIState};
use crate::ui::keyboard::{match_key, Keybindings, Keybound};
use crate::world::actors::{check_player_move_attempt, MoveAttemptResult, WorldAction};
use crate::world::types::WorldDirection;

pub fn player_in_world_controller(
    world: &mut World,
    maybe_key: Option<VirtualKeyCode>,
) -> NewStates {
    let mut new_states = NewStates {
        ui_state: UIState::PlayerInWorld,
        run_state: RunState::DeferringToUI,
    };

    let maybe_keybound;
    {
        let keybindings = world.fetch::<Keybindings>();
        maybe_keybound = match_key(&keybindings.player_in_world, maybe_key);
    }

    match maybe_keybound {
        None => (),
        Some(keybound) => {
            match keybound {
                Keybound::WorldAction(world_action) => {
                    match world_action {
                        WorldAction::Move(direction) => {
                            new_states = move_attempt(world, direction);
                        }
                        // FIXME
                        WorldAction::Pickup => (),
                        // FIXME
                        WorldAction::Wait => (),
                    }
                }
                Keybound::UIAction(ui_action) => match ui_action {
                    UIAction::OpenMenu(menu) => {
                        new_states.ui_state = UIState::ActiveMenu(menu);
                    }
                    _ => (),
                },
            }
        }
    }

    return new_states;
}

pub fn move_attempt(world: &mut World, direction: WorldDirection) -> NewStates {
    let entities = world.entities();
    let player_store = world.read_storage::<Player>();
    let mut wants_to_move_store = world.write_storage::<WantsToMove>();
    let mut wants_to_melee_store = world.write_storage::<WantsToMelee>();

    let move_attempt_result = check_player_move_attempt(world, direction);
    match move_attempt_result {
        MoveAttemptResult::MoveToFreeSpace(destination) => {
            for (player_entity, _player_component) in (&entities, &player_store).join() {
                wants_to_move_store
                    .insert(
                        player_entity,
                        WantsToMove {
                            destination: destination,
                        },
                    )
                    .expect("Queueing player move failed");
            }
            NewStates {
                ui_state: UIState::PlayerInWorld,
                run_state: RunState::WorldTick,
            }
        }
        MoveAttemptResult::AttackHostile(target) => {
            for (player_entity, _player_component) in (&entities, &player_store).join() {
                wants_to_melee_store
                    .insert(player_entity, WantsToMelee { target: target })
                    .expect("Queueing player melee attack failed");
            }
            NewStates {
                ui_state: UIState::PlayerInWorld,
                run_state: RunState::WorldTick,
            }
        }
        // FIXME: give some UI feedback
        MoveAttemptResult::Blocked => NewStates {
            ui_state: UIState::PlayerInWorld,
            run_state: RunState::WorldTick,
        },
    }
}
