extern crate derive_more;
use super::{Height, Width};
use crate::Unit;
use derive_more::{Add, Div, Mul, Sub};
use std::ops::{Add as AddTrait, Div as DivTrait, Mul as MulTrait, Sub as SubTrait};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Add, Sub, Mul, Div)]
pub struct Size2D<T: Unit> {
    pub width: Width<T>,
    pub height: Height<T>,
}

impl<T: Unit + AddTrait<T> + SubTrait<T> + MulTrait<i32> + DivTrait<i32>> Size2D<T> {
    pub fn nothing() -> Self {
        Self {
            width: Width(Unit::zero()),
            height: Height(Unit::zero()),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            width: self.width.abs(),
            height: self.height.abs(),
        }
    }
}

// Add Width to Size2D
impl<T: Unit + AddTrait<Output = T>> AddTrait<Width<T>> for Size2D<T> {
    type Output = Self;

    fn add(self, rhs: Width<T>) -> Self::Output {
        Self {
            width: self.width + rhs,
            height: self.height,
        }
    }
}

// Subtract Width from Size2D
impl<T: Unit + SubTrait<Output = T>> SubTrait<Width<T>> for Size2D<T> {
    type Output = Self;

    fn sub(self, rhs: Width<T>) -> Self::Output {
        Self {
            width: self.width - rhs,
            height: self.height,
        }
    }
}

// Add Height to Size2D
impl<T: Unit + AddTrait<Output = T>> AddTrait<Height<T>> for Size2D<T> {
    type Output = Self;

    fn add(self, rhs: Height<T>) -> Self::Output {
        Self {
            width: self.width,
            height: self.height + rhs,
        }
    }
}

// Subtract Height from Size2D
impl<T: Unit + SubTrait<Output = T>> SubTrait<Height<T>> for Size2D<T> {
    type Output = Self;

    fn sub(self, rhs: Height<T>) -> Self::Output {
        Self {
            width: self.width,
            height: self.height - rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Height, Size2D, Width};
    use crate::example::MyUnit;

    #[test]
    fn nothing_gives_0_0() {
        let nothing = Size2D::<MyUnit>::nothing();
        assert_eq!(nothing.width, Width(MyUnit(0)));
        assert_eq!(nothing.height, Height(MyUnit(0)));
    }

    #[test]
    fn abs_gives_size2d_with_positive_width_height() {
        let size2d = Size2D {
            width: Width(MyUnit(-2)),
            height: Height(MyUnit(-3)),
        };
        assert_eq!(
            size2d.abs(),
            Size2D {
                width: Width(MyUnit(2)),
                height: Height(MyUnit(3)),
            }
        );
    }

    #[test]
    fn add_width_gives_size2d() {
        let size2d = Size2D {
            width: Width(MyUnit(2)),
            height: Height(MyUnit(3)),
        };
        let width = Width(MyUnit(4));
        assert_eq!(
            size2d + width,
            Size2D {
                width: Width(MyUnit(6)),
                height: Height(MyUnit(3)),
            }
        );
    }

    #[test]
    fn sub_width_gives_size2d() {
        let size2d = Size2D {
            width: Width(MyUnit(6)),
            height: Height(MyUnit(3)),
        };
        let width = Width(MyUnit(4));
        assert_eq!(
            size2d - width,
            Size2D {
                width: Width(MyUnit(2)),
                height: Height(MyUnit(3)),
            }
        );
    }

    #[test]
    fn add_height_gives_size2d() {
        let size2d = Size2D {
            width: Width(MyUnit(2)),
            height: Height(MyUnit(3)),
        };
        let height = Height(MyUnit(4));
        assert_eq!(
            size2d + height,
            Size2D {
                width: Width(MyUnit(2)),
                height: Height(MyUnit(7)),
            }
        );
    }

    #[test]
    fn sub_height_gives_size2d() {
        let size2d = Size2D {
            width: Width(MyUnit(2)),
            height: Height(MyUnit(7)),
        };
        let height = Height(MyUnit(4));
        assert_eq!(
            size2d - height,
            Size2D {
                width: Width(MyUnit(2)),
                height: Height(MyUnit(3)),
            }
        );
    }

    #[test]
    fn add_size2d_gives_size2d() {
        let a = Size2D {
            width: Width(MyUnit(2)),
            height: Height(MyUnit(3)),
        };
        let b = Size2D {
            width: Width(MyUnit(4)),
            height: Height(MyUnit(5)),
        };
        assert_eq!(
            a + b,
            Size2D {
                width: Width(MyUnit(6)),
                height: Height(MyUnit(8)),
            }
        );
    }

    #[test]
    fn sub_size2d_gives_size2d() {
        let a = Size2D {
            width: Width(MyUnit(6)),
            height: Height(MyUnit(8)),
        };
        let b = Size2D {
            width: Width(MyUnit(4)),
            height: Height(MyUnit(5)),
        };
        assert_eq!(
            a - b,
            Size2D {
                width: Width(MyUnit(2)),
                height: Height(MyUnit(3)),
            }
        );
    }

    #[test]
    fn mul_by_i32_gives_size2d() {
        let size2d = Size2D {
            width: Width(MyUnit(2)),
            height: Height(MyUnit(3)),
        };
        assert_eq!(
            size2d * 3,
            Size2D {
                width: Width(MyUnit(6)),
                height: Height(MyUnit(9)),
            }
        );
    }

    #[test]
    fn div_by_i32_gives_size2d() {
        let size2d = Size2D {
            width: Width(MyUnit(6)),
            height: Height(MyUnit(10)),
        };
        assert_eq!(
            size2d / 3,
            Size2D {
                width: Width(MyUnit(2)),
                height: Height(MyUnit(3)),
            }
        );
    }
}
