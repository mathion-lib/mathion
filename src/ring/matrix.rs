/*
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

use crate::field::galois_field::{GaloisField as Field};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Define matrix which elements are element of arbitrary field F
struct Matrix<
    'a,
    T: Sized
        + Copy
        + Neg
        + Add<Output = T>
        + AddAssign
        + Div<Output = T>
        + DivAssign
        + Mul<Output = T>
        + MulAssign
        + Sub<Output = T>
        + SubAssign,
> {
    data: &'a [&'a [T]],
    row: usize,
    column: usize,
}

impl<'a> Matrix<'a, Field> {}
