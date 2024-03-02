use std::ops::{Div as DivTrait, Mul as MulTrait};
extern crate derive_more;
use crate::Unit;
use derive_more::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub)]
pub struct Height<T: Unit>(pub T);

impl<T: Unit> Height<T> {
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    pub fn to_primitive(&self) -> i32 {
        self.0.to_primitive()
    }
}

impl<T: Unit + MulTrait<i32, Output = T>> MulTrait<i32> for Height<T> {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl<T: Unit + DivTrait<i32, Output = T>> DivTrait<i32> for Height<T> {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::Height;
    use crate::example::MyUnit;

    #[test]
    fn add_height_gives_height() {
        assert_eq!(
            Height(MyUnit(2)) + Height(MyUnit(3)),
            Height::<MyUnit>(MyUnit(5))
        );
    }

    #[test]
    fn mul_by_i32_gives_height() {
        assert_eq!(Height(MyUnit(3)) * 2, Height(MyUnit(6)));
    }

    #[test]
    fn div_by_i32_gives_height() {
        assert_eq!(Height(MyUnit(12)) / 3, Height(MyUnit(4)));
    }
}
