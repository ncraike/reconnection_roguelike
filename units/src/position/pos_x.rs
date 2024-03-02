extern crate derive_more;
use crate::{Unit, Width};
use std::ops::{Add as AddTrait, Sub as SubTrait};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PosX<T: Unit>(pub T);

impl<T: Unit> PosX<T> {
    pub fn to_primitive(&self) -> i32 {
        self.0.to_primitive()
    }
}

// Add Width to PosX gives PosX
impl<T: Unit + AddTrait<Output = T>> AddTrait<Width<T>> for PosX<T> {
    type Output = Self;

    fn add(self, rhs: Width<T>) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

// Subtract Width from PosX gives PosX
impl<T: Unit + SubTrait<Output = T>> SubTrait<Width<T>> for PosX<T> {
    type Output = PosX<T>;

    fn sub(self, rhs: Width<T>) -> Self::Output {
        PosX::<T>(self.0 - rhs.0)
    }
}

// Subtract PosX from PosX gives Width
impl<T: Unit + SubTrait<Output = T>> SubTrait<PosX<T>> for PosX<T> {
    type Output = Width<T>;

    fn sub(self, rhs: PosX<T>) -> Self::Output {
        Width::<T>(self.0 - rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{PosX, Width};
    use crate::example::MyUnit;

    #[test]
    fn add_width_gives_pos_x() {
        assert_eq!(PosX(MyUnit(3)) + Width(MyUnit(4)), PosX(MyUnit(7)));
    }

    #[test]
    fn sub_width_gives_pos_x() {
        assert_eq!(PosX(MyUnit(7)) - Width(MyUnit(4)), PosX(MyUnit(3)));
    }

    #[test]
    fn sub_pos_x_gives_width() {
        assert_eq!(PosX(MyUnit(7)) - PosX(MyUnit(3)), Width(MyUnit(4)));
    }
}
