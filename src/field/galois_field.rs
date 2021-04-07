/*
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

use alga::general::*;
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

#[derive(Alga, Clone, Copy, Hash, Debug)]
#[alga_traits(Field(Additive, Multiplicative))]
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

    pub fn neg(&self) -> Self {
        GaloisField {
            val: -self.val,
        }
    }

    pub fn modulo(&mut self, p: i64) {
        self.val %= p;
    }
}

pub const ZERO: GaloisField = GaloisField { val: 0 };

pub const ONE: GaloisField = GaloisField { val: 1 };

impl AbstractMagma<Additive> for GaloisField {
    fn operate(&self, right: &Self) -> Self {
        Self {
            val: (((self.val as i128) + (right.val as i128)) % (DEFAULT_P as i128)) as i64,
        }
    }
}

impl Identity<Additive> for GaloisField {
    fn identity() -> Self {
        ZERO
    }
}

impl TwoSidedInverse<Additive> for GaloisField {
    fn two_sided_inverse(&self) -> Self {
        Self { val: -self.val }
    }
}

impl AbstractMagma<Multiplicative> for GaloisField {
    fn operate(&self, right: &Self) -> Self {
        Self {
            val: (((self.val as i128) * (right.val as i128)) % (DEFAULT_P as i128)) as i64,
        }
    }
}

impl Identity<Multiplicative> for GaloisField {
    fn identity() -> Self {
        ONE
    }
}

impl TwoSidedInverse<Multiplicative> for GaloisField {
    fn two_sided_inverse(&self) -> Self {
        self.pow((DEFAULT_P - 2) as usize)
    }
}

impl From<usize> for GaloisField {
    fn from(val: usize) -> Self {
        Self { val: val as i64 }
    }
}

impl From<i64> for GaloisField {
    fn from(val: i64) -> Self {
        Self { val: val }
    }
}
