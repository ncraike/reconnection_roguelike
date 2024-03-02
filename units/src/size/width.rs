use std::ops::{Div as DivTrait, Mul as MulTrait};
extern crate derive_more;
use crate::Unit;
use derive_more::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub)]
pub struct Width<T: Unit>(pub T);

impl<T: Unit> Width<T> {
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    pub fn to_primitive(&self) -> i32 {
        self.0.to_primitive()
    }
}

impl<T: Unit + MulTrait<i32, Output = T>> MulTrait<i32> for Width<T> {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl<T: Unit + DivTrait<i32, Output = T>> DivTrait<i32> for Width<T> {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::Width;
    use crate::example::MyUnit;

    #[test]
    fn add() {
        assert_eq!(
            Width(MyUnit(1)) + Width(MyUnit(2)),
            Width::<MyUnit>(MyUnit(3))
        );
    }

    #[test]
    fn mul_by_i32_gives_width() {
        assert_eq!(Width(MyUnit(4)) * 3, Width(MyUnit(12)))
    }

    #[test]
    fn div_by_i32_gives_width_() {
        assert_eq!(Width(MyUnit(8)) / 2, Width(MyUnit(4)))
    }
}
