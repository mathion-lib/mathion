/*
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

use std::cmp::{Eq, Ordering, PartialEq};
use std::convert::From;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub const DEFAULT_P: i64 = 469_762_049;

/// Calculate power in O(lgn) time by using divide-and-conquer
pub fn fast_pow(a: i64, x: usize, p: i64) -> i64 {
    let (mut r, mut a_t, mut x_t) = (1, a, x);
    while x_t != 0 {
        if (x_t & 1) == 1 {
            r = (r * a_t) % p;
        }
        a_t = (a_t * a_t) % p;
        x_t >>= 1;
    }
    r
}

#[derive(Clone, Copy, Hash, Debug)]
pub struct GaloisField {
    pub val: i64,
}

impl PartialEq for GaloisField {
    fn eq(&self, other: &GaloisField) -> bool {
        self.val == other.val
            || (DEFAULT_P + self.val) % DEFAULT_P == other.val
            || (DEFAULT_P + other.val) % DEFAULT_P == self.val
    }
}

impl Eq for GaloisField {}

impl GaloisField {
    pub fn pow(&self, exp: usize) -> Self {
        Self {
            val: fast_pow(self.val, exp, DEFAULT_P),
        }
    }

    pub fn abs(&self) -> Self {
        GaloisField {
            val: self.val.abs(),
        }
    }
}

pub const ZERO: GaloisField = GaloisField { val: 0 };

pub const ONE: GaloisField = GaloisField { val: 1 };

impl PartialOrd for GaloisField {
    fn partial_cmp(&self, other: &GaloisField) -> Option<Ordering> {
        Some(self.val.cmp(&other.val))
    }
}

impl Ord for GaloisField {
    fn cmp(&self, other: &GaloisField) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl Add for GaloisField {
    type Output = GaloisField;
    fn add(self, other: GaloisField) -> GaloisField {
        GaloisField {
            val: (((self.val as i128) + (other.val as i128)) % (DEFAULT_P as i128)) as i64,
        }
    }
}

impl AddAssign for GaloisField {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Sub for GaloisField {
    type Output = GaloisField;
    fn sub(self, other: GaloisField) -> GaloisField {
        GaloisField {
            val: (((self.val as i128) - (other.val as i128)) % (DEFAULT_P as i128)) as i64,
        }
    }
}

impl SubAssign for GaloisField {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl Mul for GaloisField {
    type Output = GaloisField;
    fn mul(self, other: GaloisField) -> GaloisField {
        GaloisField {
            val: (((self.val as i128) * (other.val as i128)) % (DEFAULT_P as i128)) as i64,
        }
    }
}

impl MulAssign for GaloisField {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl Div for GaloisField {
    type Output = GaloisField;
    fn div(self, other: GaloisField) -> GaloisField {
        GaloisField {
            val: (((self.val as i128)
                * (fast_pow(other.val, (DEFAULT_P - 2) as usize, DEFAULT_P) as i128))
                % (DEFAULT_P as i128)) as i64,
        }
    }
}

impl DivAssign for GaloisField {
    fn div_assign(&mut self, other: Self) {
        *self = self.clone() / other;
    }
}

impl Neg for GaloisField {
    type Output = GaloisField;
    fn neg(self) -> GaloisField {
        GaloisField { val: -self.val }
    }
}

impl From<i64> for GaloisField {
    fn from(target: i64) -> GaloisField {
        GaloisField { val: target }
    }
}

impl From<usize> for GaloisField {
    fn from(target: usize) -> GaloisField {
        GaloisField { val: target as i64 }
    }
}
