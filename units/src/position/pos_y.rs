extern crate derive_more;
use crate::{Height, Unit};
use std::ops::{Add as AddTrait, Sub as SubTrait};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PosY<T: Unit>(pub T);

impl<T: Unit> PosY<T> {
    pub fn to_primitive(&self) -> i32 {
        self.0.to_primitive()
    }
}

// Add Height to PosY gives PosY
impl<T: Unit + AddTrait<Output = T>> AddTrait<Height<T>> for PosY<T> {
    type Output = Self;

    fn add(self, rhs: Height<T>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

// Subtract Height from PosY gives PosY
impl<T: Unit + SubTrait<Output = T>> SubTrait<Height<T>> for PosY<T> {
    type Output = PosY<T>;

    fn sub(self, rhs: Height<T>) -> Self::Output {
        PosY::<T>(self.0 - rhs.0)
    }
}

// Subtract PosY from PosY gives Height
impl<T: Unit + SubTrait<Output = T>> SubTrait<PosY<T>> for PosY<T> {
    type Output = Height<T>;

    fn sub(self, rhs: PosY<T>) -> Self::Output {
        Height::<T>(self.0 - rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{Height, PosY};
    use crate::example::MyUnit;

    #[test]
    fn add_height_gives_pos_y() {
        assert_eq!(PosY(MyUnit(2)) + Height(MyUnit(3)), PosY(MyUnit(5)));
    }

    #[test]
    fn sub_height_gives_pos_x() {
        assert_eq!(PosY(MyUnit(5)) - Height(MyUnit(3)), PosY(MyUnit(2)));
    }

    #[test]
    fn sub_pos_y_gives_height() {
        assert_eq!(PosY(MyUnit(5)) - PosY(MyUnit(2)), Height(MyUnit(3)));
    }
}
