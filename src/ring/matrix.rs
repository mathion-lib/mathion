/*
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

use alga::general::*;
use alga::linear::Matrix;
/// Define matrix which elements are element of arbitrary field F
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatrixRing<Scalar: Field> {
    data: Box<[Box<[Scalar]>]>,
    row: usize,
    column: usize,
}

#[macro_export]
macro_rules! matrix {
    ($([$($x:expr),+]), +) => {
        {
            let mut matrix: Vec<Vec<GaloisField>> = Vec::new();
            let (mut r, mut c) = (0, 0);
            $(
                let mut row: Vec<GaloisField> = Vec::new();
                $(
                    row.push(GaloisField::from($x));
                )*
                c = row.len();
                matrix.push(row);
                r += 1;
            )*
            Matrix::new(matrix, r, c)
        }
    };
}

/// # Example
/// ```rust
/// use mathion::matrix;
/// use mathion::ring::matrix::Matrix;
/// use mathion::field::GaloisField;
/// let m: Matrix<GaloisField> = matrix![[1,2,3],[1,2,3]];
/// ```
impl Matrix<Scalar> {
    pub fn new(source: Vec<Vec<Scalar>>, row: usize, column: usize) -> Self {
        let mut data: Vec<Box<[Scalar]>> = Vec::with_capacity(row);
        for row in source {
            data.push(row.into_boxed_slice());
        }
        Self {
            data: data.into_boxed_slice(),
            row: row,
            column: column,
        }
    }
}
