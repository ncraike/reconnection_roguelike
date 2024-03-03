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
        if x > w {
            panic!("Position2D's x {} is greater than the given width {}", x, w);
        }
        (y * w) + x
    }

    pub fn from_buffer_index(index: usize, width: Width<T>) -> Self {
        Self {
            x: PosX(T::new(index as i32 % width.to_primitive())),
            y: PosY(T::new(index as i32 / width.to_primitive())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Position2D;
    use crate::example::MyUnit;
    use crate::{PosX, PosY, Width};

    #[test]
    fn to_buffer_index_pos_0_0_gives_0() {
        let pos2d = Position2D::<MyUnit> {
            x: PosX(MyUnit(0)),
            y: PosY(MyUnit(0)),
        };
        assert_eq!(pos2d.to_buffer_index(Width(MyUnit(10))), 0 as usize);
    }

    #[test]
    fn to_buffer_index_x_of_0_gives_x_if_less_than_width() {
        let pos2d = Position2D::<MyUnit> {
            x: PosX(MyUnit(9)),
            y: PosY(MyUnit(0)),
        };
        assert_eq!(pos2d.to_buffer_index(Width(MyUnit(10))), 9 as usize);
    }

    #[test]
    fn to_buffer_index_y_of_1_gives_width() {
        let pos2d = Position2D::<MyUnit> {
            x: PosX(MyUnit(0)),
            y: PosY(MyUnit(1)),
        };
        assert_eq!(pos2d.to_buffer_index(Width(MyUnit(7))), 7 as usize);
    }

    #[test]
    #[should_panic]
    fn to_buffer_index_should_panic_if_x_greater_than_width() {
        let pos2d = Position2D::<MyUnit> {
            x: PosX(MyUnit(20)),
            y: PosY(MyUnit(3)),
        };
        pos2d.to_buffer_index(Width(MyUnit(10)));
    }

    #[test]
    fn to_buffer_index_y_of_4_times_width_of_5_plus_x_of_3_equals_23() {
        let pos2d = Position2D::<MyUnit> {
            x: PosX(MyUnit(3)),
            y: PosY(MyUnit(4)),
        };
        assert_eq!(pos2d.to_buffer_index(Width(MyUnit(5))), 23 as usize);
    }

    #[test]
    fn from_buffer_index_with_index_0_gives_pos_0_0() {
        assert_eq!(
            Position2D::from_buffer_index(0, Width(MyUnit(10))),
            Position2D {
                x: PosX(MyUnit(0)),
                y: PosY(MyUnit(0)),
            }
        );
    }

    #[test]
    fn from_buffer_index_with_index_eq_width_gives_pos_0_1() {
        assert_eq!(
            Position2D::from_buffer_index(10, Width(MyUnit(10))),
            Position2D {
                x: PosX(MyUnit(0)),
                y: PosY(MyUnit(1)),
            }
        );
    }

    #[test]
    fn from_buffer_index_with_index_17_width_7_gives_pos_3_2() {
        assert_eq!(
            Position2D::from_buffer_index(17, Width(MyUnit(7))),
            Position2D {
                x: PosX(MyUnit(3)),
                y: PosY(MyUnit(2)),
            }
        );
    }
}
