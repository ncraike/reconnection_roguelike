use units::Size2DI32;

use crate::world::units::WorldUnits;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum WorldDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum WorldAction {
    Move(WorldDirection),
    Pickup,
    Wait,
}

pub fn convert_direction_to_delta(direction: WorldDirection) -> Size2DI32<WorldUnits> {
    // Remember origin is top-left
    match direction {
        WorldDirection::North => WorldUnits::new_size2d(0, -1),
        WorldDirection::NorthEast => WorldUnits::new_size2d(1, -1),
        WorldDirection::East => WorldUnits::new_size2d(1, 0),
        WorldDirection::SouthEast => WorldUnits::new_size2d(1, 1),
        WorldDirection::South => WorldUnits::new_size2d(0, 1),
        WorldDirection::SouthWest => WorldUnits::new_size2d(-1, 1),
        WorldDirection::West => WorldUnits::new_size2d(-1, 0),
        WorldDirection::NorthWest => WorldUnits::new_size2d(-1, -1),
    }
}
