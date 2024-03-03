use crate::{PosX, PosY, Unit, Width};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position2D<T: Unit> {
    pub x: PosX<T>,
    pub y: PosY<T>,
}

impl<T: Unit> Position2D<T> {
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
}
