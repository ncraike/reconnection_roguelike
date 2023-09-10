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
