use bracket_geometry::prelude::Point;

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

// FIXME: consider not using Point for distance. Ressurect ultraviolet Vec2i?
pub fn convert_direction_to_delta(direction: WorldDirection) -> Point {
    // Remember origin is top-left
    match direction {
        WorldDirection::North => Point { x: 0, y: -1 },
        WorldDirection::NorthEast => Point { x: 1, y: -1 },
        WorldDirection::East => Point { x: 1, y: 0 },
        WorldDirection::SouthEast => Point { x: 1, y: 1 },
        WorldDirection::South => Point { x: 0, y: 1 },
        WorldDirection::SouthWest => Point { x: -1, y: 1 },
        WorldDirection::West => Point { x: -1, y: 0 },
        WorldDirection::NorthWest => Point { x: -1, y: -1 },
    }
}
