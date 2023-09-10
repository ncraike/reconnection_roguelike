use super::types::WorldDirection;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Action {
    Move(WorldDirection),
    MeleeAttack(WorldDirection),
    Pickup,
    Wait,
}
