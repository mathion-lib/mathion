/*
    Author : quickn (quickn.ga)
    Email  : quickwshell@gmail.com
*/

use crate::polynomial::IntPolynomial;
use std::cmp::max;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BigInteger<'a> {
    data: &'a [u64],
    // false : positive, true : negative
    sign: bool,
}

pub const ZERO: BigInteger = BigInteger {
    data: &[0],
    sign: false,
};

pub const ONE: BigInteger = BigInteger {
    data: &[1],
    sign: false,
};

impl<'a> BigInteger<'a> {
    pub fn new(data: Vec<u64>, sign: bool) -> Self {
        let (ptr, len, _) = data.into_raw_parts();
        Self {
            data: unsafe { std::slice::from_raw_parts(ptr, len) },
            sign: sign,
        }
    }
}

const MOD: u128 = 1 << BASE_BITS;
const BASE_BITS: u8 = 64;

impl Add for BigInteger<'_> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let new_sz = max(self.data.len(), other.data.len());
        let mut target = self.clone().data.to_vec();
        let mut source = other.clone().data.to_vec();
        target.resize(new_sz, 0);
        source.resize(new_sz, 0);
        let mut round: u64 = 0;
        for i in 0..new_sz {
            let tmp = (target[i] as u128) + (source[i] as u128) + (round as u128);
            round = (tmp >> BASE_BITS) as u64;
            target[i] = (tmp % MOD) as u64;
        }
        if round != 0 {
            target.push(round);
        }
        Self::new(target.clone(), self.sign)
    }
}

impl AddAssign for BigInteger<'_> {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Sub for BigInteger<'_> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut target = other.clone();
        {
            let mut new_data = target.data.clone().to_vec();
            let new_len = new_data.len();
            for i in 0..new_len {
                new_data[i] = !new_data[i];
            }
            let (ptr, len, _) = new_data.into_raw_parts();
            target.data = unsafe { std::slice::from_raw_parts(ptr, len) };
            target += ONE;
        }
        let mut res = self + target;
        {
            let mut new_data = res.data.clone().to_vec();
            let new_len = new_data.len();
            let top = new_data[new_len - 1];
            if top == 1 {
                new_data.remove(new_len - 1);
            } else {
                let mut bit_len = BASE_BITS;
                while bit_len > 0 && top & (1 << (bit_len - 1)) == 0 {
                    bit_len -= 1;
                }
                if bit_len == 0 {
                    new_data.remove(new_len - 1);
                    dbg!(new_data.clone());
                    if new_len >= 2 {
                        let top2 = new_data[new_len-2];
                        let mut bit_len2 = BASE_BITS;
                        while bit_len2 > 0 && top2 & (1 << (bit_len2 - 1)) == 0 {
                            bit_len2 -= 1;
                        }
                        new_data[new_len-2] ^= 1 << (bit_len2 - 1);
                    }
                } else {
                    new_data[new_len - 1] ^= 1 << (bit_len - 1);
                }
            }
            let (ptr, len, _) = new_data.into_raw_parts();
            res.data = unsafe { std::slice::from_raw_parts(ptr, len) };
        }
        res
    }
}

impl SubAssign for BigInteger<'_> {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}
