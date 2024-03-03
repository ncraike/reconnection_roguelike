use super::Box2D;
use crate::{Height, Unit, Width};
use std::ops::{Add, Sub};

impl<T: Unit + Copy + Add<Output = T> + Sub<Output = T> + Ord> Box2D<T> {
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
}

#[cfg(test)]
mod tests {
    use super::{Box2D, Height, Width};
    use crate::example::MyUnit;
    use crate::{PosX, PosY, Position2D};

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
