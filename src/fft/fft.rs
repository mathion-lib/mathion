/*
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

use crate::field::galois_field::{self, GaloisField, DEFAULT_P};
use alga::general::wrapper::Wrapper as W;
use alga::general::*;

const DEFAULT_G: GaloisField = GaloisField { val: 3 };
const DEFAULT_G_INV: GaloisField = GaloisField { val: 156587350 };

#[derive(Clone, Copy, Debug)]
pub enum FFTType {
    Straight,
    Inverse,
}

pub trait FFT: Sized + Copy {
    type F: Sized + Copy + From<Self> + AbstractField;
    const ZERO: Self;
    fn get_1s_roots(n: usize, fft_type: FFTType) -> W<Self::F, Additive, Multiplicative>;
    // Return multiplicative identity in GF(DEFAULT_P)
    fn type_cast(n: usize) -> Self;
}

impl FFT for GaloisField {
    type F = GaloisField;
    const ZERO: GaloisField = galois_field::ZERO;
    fn get_1s_roots(n: usize, fft_type: FFTType) -> W<Self::F, Additive, Multiplicative> {
        match fft_type {
            FFTType::Straight => W::new(DEFAULT_G.pow(((DEFAULT_P - 1) as usize) / n)),
            _ => W::new(DEFAULT_G_INV.pow(((DEFAULT_P - 1) as usize) / n)),
        }
    }

    fn type_cast(n: usize) -> Self {
        Self::from(n)
    }
}

pub fn do_fft<T: FFT>(arr: &mut [W<T::F, Additive, Multiplicative>], fft_type: FFTType) {
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
            let mut omega: W<T::F, Additive, Multiplicative> =
                W::new(Identity::<Multiplicative>::identity());
            for j in 0..(m >> 1) {
                let t: W<T::F, Additive, Multiplicative> = omega * arr[k + j + (m >> 1)];
                arr[k + j + (m >> 1)] = arr[k + j] - t.two_sided_inverse();
                arr[k + j] = arr[k + j] + t;
                omega = omega * omega_m;
            }
        }
    }
    match fft_type {
        FFTType::Inverse => {
            for i in 0..n {
                arr[i] = arr[i] * W::new(T::F::from(T::type_cast(n)));
            }
        }
        _ => {}
    }
}
