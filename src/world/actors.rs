use super::types::WorldDirection;

const STANDARD_ACTION_RECOVERY: u64 = 60;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Action {
    Move(WorldDirection),
    MeleeAttack(WorldDirection),
    Pickup,
    Wait,
}

pub fn get_action_recovery(action: Action) -> u64 {
    match action {
        Action::Move(_) => STANDARD_ACTION_RECOVERY,
        Action::MeleeAttack(_) => STANDARD_ACTION_RECOVERY,
        Action::Pickup => STANDARD_ACTION_RECOVERY,
        Action::Wait => STANDARD_ACTION_RECOVERY / 12,
    }
}
