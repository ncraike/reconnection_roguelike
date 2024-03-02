extern crate derive_more;
use crate::Unit;
use derive_more::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Mul, Div)]
pub struct Height<T: Unit>(pub T);

impl<T: Unit> Height<T> {
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    pub fn to_primitive(&self) -> i32 {
        self.0.to_primitive()
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
    fn sub_height_gives_height() {
        assert_eq!(
            Height(MyUnit(5)) - Height(MyUnit(3)),
            Height::<MyUnit>(MyUnit(2))
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
