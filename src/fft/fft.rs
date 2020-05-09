/*
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

use crate::field::galois_field::{self as field, GaloisField as Field, DEFAULT_P};

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

const DEFAULT_G: Field = Field { val: 3 };
const DEFAULT_G_INV: Field = Field { val: 156587350 };

#[derive(Clone, Copy, Debug)]
pub enum FFTType {
    Straight,
    Inverse,
}

pub trait FFT: Sized + Copy {
    type F: Sized
        + Copy
        + From<Self>
        + Neg
        + Add<Output = Self::F>
        + AddAssign
        + Div<Output = Self::F>
        + DivAssign
        + Mul<Output = Self::F>
        + MulAssign
        + Sub<Output = Self::F>
        + SubAssign;
    const ZERO: Self;
    fn get_1s_roots(n: usize, fft_type: FFTType) -> Self::F;
    // Return multiplicative identity in GF(DEFAULT_P)
    fn identity() -> Self::F;
    fn type_cast(n: usize) -> Self;
}

impl FFT for Field {
    type F = Field;
    const ZERO: Field = field::ZERO;
    fn get_1s_roots(n: usize, fft_type: FFTType) -> Self::F {
        match fft_type {
            FFTType::Straight => DEFAULT_G.pow(((DEFAULT_P - 1) as usize) / n),
            _ => DEFAULT_G_INV.pow(((DEFAULT_P - 1) as usize) / n),
        }
    }
    // Return multiplicative identity in GF(DEFAULT_P)
    fn identity() -> Self::F {
        field::ONE
    }

    fn type_cast(n: usize) -> Self {
        Self::from(n)
    }
}

pub fn do_fft<T: FFT>(arr: &mut [T::F], fft_type: FFTType) {
    let n = arr.len();
    // Bit reverse stage
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j >= bit {
            j -= bit;
            bit >>= 1;
        }
        j += bit;
        if i < j {
            arr.swap(i, j);
        }
    }
    // Do fast-fourier-transform by using cooley-tukey algorithm
    for l in 1.. {
        let m = 1 << l;
        if m > n {
            break;
        }
        let omega_m = T::get_1s_roots(m, fft_type);
        for k in (0..n).step_by(m) {
            let mut omega = T::identity();
            for j in 0..(m >> 1) {
                let t = omega * arr[k + j + (m >> 1)];
                arr[k + j + (m >> 1)] = arr[k + j] - t;
                arr[k + j] += t;
                omega *= omega_m;
            }
        }
    }
    match fft_type {
        FFTType::Inverse => {
            for i in 0..n {
                arr[i] /= T::F::from(T::type_cast(n));
            }
        }
        _ => {}
    }
}
