use super::{PosX, PosY};
use crate::{Height, Size2D, Unit, Width};
use bracket_geometry::prelude::Point;
use std::ops::{Add as AddTrait, Sub as SubTrait};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position2D<T: Unit> {
    pub x: PosX<T>,
    pub y: PosY<T>,
}

impl<T: Unit + Copy + AddTrait<Output = T> + SubTrait<Output = T> + Ord> Position2D<T> {
    pub fn origin() -> Self {
        Self {
            x: PosX(T::zero()),
            y: PosY(T::zero()),
        }
    }

    pub fn with_x(self, new_x: PosX<T>) -> Self {
        Self {
            x: new_x,
            y: self.y,
        }
    }

    pub fn with_x_of(self, other_position: Self) -> Self {
        self.with_x(other_position.x)
    }

    pub fn with_y(self, new_y: PosY<T>) -> Self {
        Self {
            x: self.x,
            y: new_y,
        }
    }

    pub fn with_y_of(self, other_position: Self) -> Self {
        self.with_y(other_position.y)
    }

    pub fn to_buffer_index(self, width: Width<T>) -> usize {
        let x: usize = self.x.to_primitive().try_into().ok().unwrap();
        let y: usize = self.y.to_primitive().try_into().ok().unwrap();
        let w: usize = width.to_primitive().try_into().ok().unwrap();
        (y * w) + x
    }

    pub fn from_buffer_index(index: usize, width: Width<T>) -> Self {
        Self {
            x: PosX(T::new(index as i32 % width.to_primitive())),
            y: PosY(T::new(index as i32 / width.to_primitive())),
        }
    }

    pub fn to_bracket_geometry_point(self) -> Point {
        Point {
            x: self.x.0.to_primitive(),
            y: self.y.0.to_primitive(),
        }
    }

    pub fn from_bracket_geometry_point(point: Point) -> Self {
        Self {
            x: PosX(T::new(point.x)),
            y: PosY(T::new(point.y)),
        }
    }
}

// Add Width gives Position2D
impl<T: Unit + AddTrait<Output = T>> AddTrait<Width<T>> for Position2D<T> {
    type Output = Self;

    fn add(self, rhs: Width<T>) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y,
        }
    }
}

// Subtract Width gives Position2D
impl<T: Unit + SubTrait<Output = T>> SubTrait<Width<T>> for Position2D<T> {
    type Output = Self;

    fn sub(self, rhs: Width<T>) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y,
        }
    }
}

// Add Height gives Position2D
impl<T: Unit + AddTrait<Output = T>> AddTrait<Height<T>> for Position2D<T> {
    type Output = Self;

    fn add(self, rhs: Height<T>) -> Self::Output {
        Self {
            x: self.x,
            y: self.y + rhs,
        }
    }
}

// Subtract Height gives Position2D
impl<T: Unit + SubTrait<Output = T>> SubTrait<Height<T>> for Position2D<T> {
    type Output = Self;

    fn sub(self, rhs: Height<T>) -> Self::Output {
        Self {
            x: self.x,
            y: self.y - rhs,
        }
    }
}

// Add Size2D gives Position2D
impl<T: Unit + AddTrait<Output = T>> AddTrait<Size2D<T>> for Position2D<T> {
    type Output = Self;

    fn add(self, rhs: Size2D<T>) -> Self::Output {
        Self {
            x: self.x + rhs.width,
            y: self.y + rhs.height,
        }
    }
}

// Subtract Size2D gives Position2D
impl<T: Unit + SubTrait<Output = T>> SubTrait<Size2D<T>> for Position2D<T> {
    type Output = Position2D<T>;

    fn sub(self, rhs: Size2D<T>) -> Position2D<T> {
        Position2D::<T> {
            x: self.x - rhs.width,
            y: self.y - rhs.height,
        }
    }
}

// Subtract Position2D gives Size2D
impl<T: Unit + SubTrait<Output = T>> SubTrait<Position2D<T>> for Position2D<T> {
    type Output = Size2D<T>;

    fn sub(self, rhs: Position2D<T>) -> Size2D<T> {
        Size2D::<T> {
            width: self.x - rhs.x,
            height: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Height, PosX, PosY, Position2D, Size2D, Width};
    use crate::example::MyUnit;

    #[test]
    fn add_size2d_gives_position2d() {
        let pos2d = Position2D::<MyUnit> {
            x: PosX(MyUnit(2)),
            y: PosY(MyUnit(3)),
        };
        let size2d = Size2D::<MyUnit> {
            width: Width(MyUnit(4)),
            height: Height(MyUnit(5)),
        };
        assert_eq!(
            pos2d + size2d,
            Position2D::<MyUnit> {
                x: PosX(MyUnit(6)),
                y: PosY(MyUnit(8)),
            }
        );
    }

    #[test]
    fn sub_position_2d_gives_size_2d() {
        let pos1 = Position2D::<MyUnit> {
            x: PosX(MyUnit(2)),
            y: PosY(MyUnit(3)),
        };
        let pos2 = Position2D::<MyUnit> {
            x: PosX(MyUnit(6)),
            y: PosY(MyUnit(8)),
        };
        assert_eq!(
            pos2 - pos1,
            Size2D::<MyUnit> {
                width: Width(MyUnit(4)),
                height: Height(MyUnit(5)),
            }
        );
    }

    #[test]
    fn add_width_gives_position_2d() {
        let pos2d = Position2D::<MyUnit> {
            x: PosX(MyUnit(2)),
            y: PosY(MyUnit(3)),
        };
        let width = Width(MyUnit(4));
        assert_eq!(
            pos2d + width,
            Position2D::<MyUnit> {
                x: PosX(MyUnit(6)),
                y: PosY(MyUnit(3)),
            }
        );
    }

    #[test]
    fn sub_width_gives_position_2d() {
        let pos2d = Position2D::<MyUnit> {
            x: PosX(MyUnit(6)),
            y: PosY(MyUnit(3)),
        };
        let width = Width(MyUnit(4));
        assert_eq!(
            pos2d - width,
            Position2D::<MyUnit> {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            }
        );
    }

    #[test]
    fn add_height_gives_position_2d() {
        let pos2d = Position2D::<MyUnit> {
            x: PosX(MyUnit(2)),
            y: PosY(MyUnit(3)),
        };
        let height = Height(MyUnit(5));
        assert_eq!(
            pos2d + height,
            Position2D::<MyUnit> {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(8)),
            }
        );
    }

    #[test]
    fn sub_height_gives_position_2d() {
        let pos2d = Position2D::<MyUnit> {
            x: PosX(MyUnit(2)),
            y: PosY(MyUnit(8)),
        };
        let height = Height(MyUnit(5));
        assert_eq!(
            pos2d - height,
            Position2D::<MyUnit> {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            }
        );
    }
}
