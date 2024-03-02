use crate::{Height, PosX, PosY, Position2D, Size2D, Unit, Width};
use bracket_geometry::prelude::Rect;
use std::cmp::{max, min};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Box2D<T: Unit> {
    pub p1: Position2D<T>,
    pub p2: Position2D<T>,
}

impl<
        T: Unit
            + Copy
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<i32, Output = T>
            + Div<i32, Output = T>
            + Ord,
    > Box2D<T>
{
    pub fn new_from_position_and_size(position: Position2D<T>, size: Size2D<T>) -> Self {
        Self {
            p1: position,
            p2: position + size,
        }
    }

    pub fn new_from_size(size: Size2D<T>) -> Self {
        Self::new_from_position_and_size(Position2D::origin(), size)
    }

    pub fn new_from_width_height(width: Width<T>, height: Height<T>) -> Self {
        Self::new_from_size(Size2D {
            width: width,
            height: height,
        })
    }

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

    pub fn center(&self) -> Position2D<T> {
        self.p1 + (self.size() / 2)
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
            && position.x <= self.x2()
            && self.y1() <= position.y
            && position.y <= self.y2()
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

    pub fn split_from_left(&self, offset_from_left: Width<T>) -> (Box2D<T>, Box2D<T>) {
        let right_box = Self {
            p1: self.p1 + offset_from_left,
            p2: self.p2,
        };
        let left_box = Self {
            p1: self.p1,
            p2: self.p2.with_x_of(right_box.p1),
        };
        (left_box, right_box)
    }

    pub fn split_from_right(&self, offset_from_right: Width<T>) -> (Box2D<T>, Box2D<T>) {
        let left_box = Self {
            p1: self.p1,
            p2: self.p2 - offset_from_right,
        };
        let right_box = Self {
            p1: self.p1.with_x_of(left_box.p2),
            p2: self.p2,
        };
        (left_box, right_box)
    }

    pub fn split_from_top(&self, offset_from_top: Height<T>) -> (Box2D<T>, Box2D<T>) {
        let bottom_box = Self {
            p1: self.p1 + offset_from_top,
            p2: self.p2,
        };
        let top_box = Self {
            p1: self.p1,
            p2: self.p2.with_y_of(bottom_box.p1),
        };
        (top_box, bottom_box)
    }

    pub fn split_from_bottom(&self, offset_from_bottom: Height<T>) -> (Box2D<T>, Box2D<T>) {
        let top_box = Self {
            p1: self.p1,
            p2: self.p2 - offset_from_bottom,
        };
        let bottom_box = Self {
            p1: self.p1.with_y_of(top_box.p2),
            p2: self.p2,
        };
        (top_box, bottom_box)
    }

    pub fn to_bracket_geometry_rect(self) -> Rect {
        Rect::with_exact(
            self.x1().0.to_primitive(),
            self.y1().0.to_primitive(),
            self.x2().0.to_primitive(),
            self.y2().0.to_primitive(),
        )
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

    #[test]
    fn split_from_left() {
        let orig_box = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
            p2: Position2D {
                x: PosX(MyUnit(7)),
                y: PosY(MyUnit(8)),
            },
        };
        let (left_box, right_box) = orig_box.split_from_left(Width(MyUnit(2)));
        assert_eq!(left_box.p1, orig_box.p1);
        assert_eq!(right_box.p2, orig_box.p2);
        assert_eq!(left_box.width(), Width(MyUnit(2)));
        assert_eq!(left_box.width() + right_box.width(), orig_box.width());
        assert_eq!(left_box.height(), orig_box.height());
        assert_eq!(right_box.height(), orig_box.height());
        assert_eq!(
            left_box,
            Box2D {
                p1: Position2D {
                    x: PosX(MyUnit(2)),
                    y: PosY(MyUnit(3)),
                },
                p2: Position2D {
                    x: PosX(MyUnit(4)),
                    y: PosY(MyUnit(8)),
                },
            }
        );
        assert_eq!(
            right_box,
            Box2D {
                p1: Position2D {
                    x: PosX(MyUnit(4)),
                    y: PosY(MyUnit(3)),
                },
                p2: Position2D {
                    x: PosX(MyUnit(7)),
                    y: PosY(MyUnit(8)),
                },
            }
        );
    }

    #[test]
    fn split_from_right() {
        let orig_box = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
            p2: Position2D {
                x: PosX(MyUnit(7)),
                y: PosY(MyUnit(8)),
            },
        };
        let (left_box, right_box) = orig_box.split_from_right(Width(MyUnit(2)));
        assert_eq!(left_box.p1, orig_box.p1);
        assert_eq!(right_box.p2, orig_box.p2);
        assert_eq!(right_box.width(), Width(MyUnit(2)));
        assert_eq!(left_box.width() + right_box.width(), orig_box.width());
        assert_eq!(left_box.height(), orig_box.height());
        assert_eq!(right_box.height(), orig_box.height());
        assert_eq!(
            left_box,
            Box2D {
                p1: Position2D {
                    x: PosX(MyUnit(2)),
                    y: PosY(MyUnit(3)),
                },
                p2: Position2D {
                    x: PosX(MyUnit(5)),
                    y: PosY(MyUnit(8)),
                },
            }
        );
        assert_eq!(
            right_box,
            Box2D {
                p1: Position2D {
                    x: PosX(MyUnit(5)),
                    y: PosY(MyUnit(3)),
                },
                p2: Position2D {
                    x: PosX(MyUnit(7)),
                    y: PosY(MyUnit(8)),
                },
            }
        );
    }

    #[test]
    fn split_from_top() {
        let orig_box = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
            p2: Position2D {
                x: PosX(MyUnit(7)),
                y: PosY(MyUnit(9)),
            },
        };
        let (top_box, bottom_box) = orig_box.split_from_top(Height(MyUnit(2)));
        assert_eq!(top_box.p1, orig_box.p1);
        assert_eq!(bottom_box.p2, orig_box.p2);
        assert_eq!(top_box.height(), Height(MyUnit(2)));
        assert_eq!(top_box.height() + bottom_box.height(), orig_box.height());
        assert_eq!(top_box.width(), orig_box.width());
        assert_eq!(bottom_box.width(), orig_box.width());
        assert_eq!(
            top_box,
            Box2D {
                p1: Position2D {
                    x: PosX(MyUnit(2)),
                    y: PosY(MyUnit(3)),
                },
                p2: Position2D {
                    x: PosX(MyUnit(7)),
                    y: PosY(MyUnit(5)),
                },
            }
        );
        assert_eq!(
            bottom_box,
            Box2D {
                p1: Position2D {
                    x: PosX(MyUnit(2)),
                    y: PosY(MyUnit(5)),
                },
                p2: Position2D {
                    x: PosX(MyUnit(7)),
                    y: PosY(MyUnit(9)),
                },
            }
        );
    }

    #[test]
    fn split_from_bottom() {
        let orig_box = Box2D::<MyUnit> {
            p1: Position2D {
                x: PosX(MyUnit(2)),
                y: PosY(MyUnit(3)),
            },
            p2: Position2D {
                x: PosX(MyUnit(7)),
                y: PosY(MyUnit(9)),
            },
        };
        let (top_box, bottom_box) = orig_box.split_from_bottom(Height(MyUnit(2)));
        assert_eq!(top_box.p1, orig_box.p1);
        assert_eq!(bottom_box.p2, orig_box.p2);
        assert_eq!(bottom_box.height(), Height(MyUnit(2)));
        assert_eq!(top_box.height() + bottom_box.height(), orig_box.height());
        assert_eq!(top_box.width(), orig_box.width());
        assert_eq!(bottom_box.width(), orig_box.width());
        assert_eq!(
            top_box,
            Box2D {
                p1: Position2D {
                    x: PosX(MyUnit(2)),
                    y: PosY(MyUnit(3)),
                },
                p2: Position2D {
                    x: PosX(MyUnit(7)),
                    y: PosY(MyUnit(7)),
                },
            }
        );
        assert_eq!(
            bottom_box,
            Box2D {
                p1: Position2D {
                    x: PosX(MyUnit(2)),
                    y: PosY(MyUnit(7)),
                },
                p2: Position2D {
                    x: PosX(MyUnit(7)),
                    y: PosY(MyUnit(9)),
                },
            }
        );
    }
}
