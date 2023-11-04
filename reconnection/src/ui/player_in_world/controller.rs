use crate::components::{Player, WantsToMelee, WantsToMove, WantsToPickupItem};
use crate::message_log::MessageLog;
use crate::types::{RunState, UITask};
use crate::ui::common::{NewStates, UIAction, UIState};
use crate::ui::keyboard::{match_key, Keybindings, Keybound};
use crate::world::player::{
    check_player_move_attempt, check_player_pickup_attempt, MoveAttemptResult, PickupAttemptResult,
};
use crate::world::types::{WorldAction, WorldDirection};
use bracket_terminal::prelude::VirtualKeyCode;
use specs::prelude::*;

pub fn player_in_world_controller(
    world: &mut World,
    maybe_key: Option<VirtualKeyCode>,
) -> NewStates {
    let mut new_states = NewStates {
        ui_state: UIState::PlayerInWorld,
        run_state: RunState::DeferToUIFor(UITask::GetPlayerAction),
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
                        WorldAction::Pickup => {
                            new_states = pickup_attempt(world);
                        }
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
    let mut messages = world.fetch_mut::<MessageLog>();

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
        MoveAttemptResult::Blocked => {
            messages.entries.push("You can't move there.".to_string());
            NewStates {
                ui_state: UIState::PlayerInWorld,
                run_state: RunState::DeferToUIFor(UITask::GetPlayerAction),
            }
        }
    }
}

pub fn pickup_attempt(world: &mut World) -> NewStates {
    let player_entity = world.fetch::<Entity>();
    let mut messages = world.fetch_mut::<MessageLog>();
    let mut wants_to_pickup_store = world.write_storage::<WantsToPickupItem>();

    match check_player_pickup_attempt(world) {
        PickupAttemptResult::NothingToPickup => {
            messages
                .entries
                .push("There is nothing here to pick up.".to_string());
            NewStates {
                ui_state: UIState::PlayerInWorld,
                run_state: RunState::DeferToUIFor(UITask::GetPlayerAction),
            }
        }
        PickupAttemptResult::ItemAvailable(item) => {
            wants_to_pickup_store
                .insert(
                    *player_entity,
                    WantsToPickupItem {
                        collected_by: *player_entity,
                        item: item,
                    },
                )
                .expect("Unable to add want-to-pickup");
            NewStates {
                ui_state: UIState::PlayerInWorld,
                run_state: RunState::WorldTick,
            }
        }
    }
}
