extern crate derive_more;
use crate::Unit;
use derive_more::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Mul, Div)]
pub struct Width<T: Unit>(pub T);

impl<T: Unit> Width<T> {
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    pub fn to_primitive(&self) -> i32 {
        self.0.to_primitive()
    }
}

#[cfg(test)]
mod tests {
    use super::Width;
    use crate::example::MyUnit;

    #[test]
    fn add_width_gives_width() {
        assert_eq!(
            Width(MyUnit(1)) + Width(MyUnit(2)),
            Width::<MyUnit>(MyUnit(3))
        );
    }

    #[test]
    fn sub_width_gives_width() {
        assert_eq!(
            Width(MyUnit(3)) - Width(MyUnit(2)),
            Width::<MyUnit>(MyUnit(1))
        );
    }

    #[test]
    fn mul_by_i32_gives_width() {
        assert_eq!(Width(MyUnit(4)) * 3, Width(MyUnit(12)))
    }

    #[test]
    fn div_by_i32_gives_width() {
        assert_eq!(Width(MyUnit(8)) / 2, Width(MyUnit(4)))
    }
}
