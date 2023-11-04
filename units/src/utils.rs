// Stand-in operators until stable Rust has int_roundings feature
// See https://github.com/rust-lang/rust/issues/88581
pub fn div_floor(lhs: i32, rhs: i32) -> i32 {
    lhs / rhs
}

pub fn div_ceil(lhs: i32, rhs: i32) -> i32 {
    lhs / rhs + (lhs % rhs).signum()
}
