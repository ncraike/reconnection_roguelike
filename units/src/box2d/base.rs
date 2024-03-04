use crate::{Height, PosX, PosY, Position2D, Size2D, Unit, Width};
use std::cmp::{max, min};
use std::ops::{Add, Div, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Box2D<T: Unit> {
    pub p1: Position2D<T>,
    pub p2: Position2D<T>,
}

impl<T: Unit + Copy + Add<Output = T> + Sub<Output = T> + Ord> Box2D<T> {
    pub fn x1(&self) -> PosX<T> {
        self.p1.x
    }

    pub fn y1(&self) -> PosY<T> {
        self.p1.y
    }

    pub fn x2(&self) -> PosX<T> {
        self.p2.x
    }

    pub fn y2(&self) -> PosY<T> {
        self.p2.y
    }

    pub fn size(&self) -> Size2D<T> {
        self.p2 - self.p1
    }

    pub fn width(&self) -> Width<T> {
        self.size().width
    }

    pub fn height(&self) -> Height<T> {
        self.size().height
    }

    pub fn contains(&self, position: Position2D<T>) -> bool {
        self.x1() <= position.x
            && position.x < self.x2()
            && self.y1() <= position.y
            && position.y < self.y2()
    }

    pub fn normalize(&self) -> Self {
        Self {
            p1: Position2D::<T> {
                x: min(self.p1.x, self.p2.x),
                y: min(self.p1.y, self.p2.y),
            },
            p2: Position2D::<T> {
                x: max(self.p1.x, self.p2.x),
                y: max(self.p1.y, self.p2.y),
            },
        }
    }

    /// Calls a function for each x/y position in the box
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Position2D<T>),
    {
        for y in self.y1().to_primitive()..self.y2().to_primitive() {
            for x in self.x1().to_primitive()..self.x2().to_primitive() {
                f(Position2D {
                    x: PosX(T::new(x)),
                    y: PosY(T::new(y)),
                });
            }
        }
    }
}

impl<T: Unit + Copy + Add<Output = T> + Sub<Output = T> + Div<i32, Output = T> + Ord> Box2D<T> {
    pub fn center(&self) -> Position2D<T> {
        self.p1 + (self.size() / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::{Box2D, Height, PosX, PosY, Position2D, Size2D, Width};
    use crate::example::MyUnit;

    #[test]
    fn x1_y1_x2_y2() {
        let box2d = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
            p2: Position2D {
                x: PosX(MyUnit(4)),
                y: PosY(MyUnit(5)),
            },
        };
        assert_eq!(box2d.x1(), PosX(MyUnit(2)));
        assert_eq!(box2d.y1(), PosY(MyUnit(3)));
        assert_eq!(box2d.x2(), PosX(MyUnit(4)));
        assert_eq!(box2d.y2(), PosY(MyUnit(5)));
    }

    #[test]
    fn size_width_height() {
        let box2d = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
            p2: Position2D {
                x: PosX(MyUnit(4)),
                y: PosY(MyUnit(6)),
            },
        };
        assert_eq!(
            box2d.size(),
            Size2D {
                width: Width(MyUnit(2)),
                height: Height(MyUnit(3)),
            }
        );
        assert_eq!(box2d.width(), Width(MyUnit(2)));
        assert_eq!(box2d.height(), Height(MyUnit(3)));
    }

    #[test]
    fn contains_position_inside() {
        let box2d = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
            p2: Position2D {
                x: PosX(MyUnit(4)),
                y: PosY(MyUnit(6)),
            },
        };
        assert!(box2d.contains(Position2D {
            x: PosX(MyUnit(3)),
            y: PosY(MyUnit(4)),
        }));
    }

    #[test]
    fn contains_p1() {
        let box2d = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
            p2: Position2D {
                x: PosX(MyUnit(4)),
                y: PosY(MyUnit(6)),
            },
        };
        assert!(box2d.contains(box2d.p1));
    }

    #[test]
    fn contains_does_not_contain_position_outside() {
        let box2d = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
            p2: Position2D {
                x: PosX(MyUnit(4)),
                y: PosY(MyUnit(6)),
            },
        };
        assert!(!box2d.contains(Position2D {
            x: PosX(MyUnit(7)),
            y: PosY(MyUnit(8)),
        }));
    }

    #[test]
    fn contains_does_not_contain_p2() {
        let box2d = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
            p2: Position2D {
                x: PosX(MyUnit(4)),
                y: PosY(MyUnit(6)),
            },
        };
        assert!(!box2d.contains(box2d.p2));
    }

    #[test]
    fn contains_does_not_contain_p2_x() {
        let box2d = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
            p2: Position2D {
                x: PosX(MyUnit(4)),
                y: PosY(MyUnit(6)),
            },
        };
        assert!(!box2d.contains(box2d.p1.with_x_of(box2d.p2)));
    }

    #[test]
    fn contains_does_not_contain_p2_y() {
        let box2d = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
            p2: Position2D {
                x: PosX(MyUnit(4)),
                y: PosY(MyUnit(6)),
            },
        };
        assert!(!box2d.contains(box2d.p1.with_y_of(box2d.p2)));
    }

    #[test]
    fn normalize_leaves_normal_unchanged() {
        let box2d = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
            p2: Position2D {
                x: PosX(MyUnit(4)),
                y: PosY(MyUnit(5)),
            },
        };
        assert_eq!(box2d.normalize(), box2d);
    }

    #[test]
    fn normalize_swaps_p1_p2() {
        let box2d = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(4)),
                y: PosY(MyUnit(5)),
            },
            p2: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
        };
        let normal_box2d = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
            p2: Position2D {
                x: PosX(MyUnit(4)),
                y: PosY(MyUnit(5)),
            },
        };
        assert_eq!(box2d.normalize(), normal_box2d);
    }

    #[test]
    fn normalize_picks_top_left_bottom_right_corners() {
        let box2d = Box2D::<MyUnit> {
            // Bottom-left corner (assuming top-left origin)
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(5)),
            },
            // Top-right corner (assuming top-left origin)
            p2: Position2D {
                x: PosX(MyUnit(4)),
                y: PosY(MyUnit(3)),
            },
        };
        let normal_box2d = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
            p2: Position2D {
                x: PosX(MyUnit(4)),
                y: PosY(MyUnit(5)),
            },
        };
        assert_eq!(box2d.normalize(), normal_box2d);
    }
}
