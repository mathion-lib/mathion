/*
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

use std::cmp::max;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Sign {
    Negative,
    Positive,
}

/// # Example
/// ## Subtraction example
/// ```rust
/// use mathion::ring::big_integer::{Sign, BigInteger};
/// assert_eq!(
///     BigInteger::new(vec![12351838, 32], Sign::Positive) - BigInteger::new(vec![32, 32], Sign::Positive),
///     BigInteger::new(vec![12351806, 0], Sign::Positive)
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BigInteger {
    data: Vec<u64>,
    sign: Sign,
}

lazy_static! {
    pub static ref ZERO: BigInteger = BigInteger {
        data: vec![0],
        sign: Sign::Positive,
    };
    pub static ref ONE: BigInteger = BigInteger {
        data: vec![1],
        sign: Sign::Positive,
    };
}

impl BigInteger {
    pub fn new(data: Vec<u64>, sign: Sign) -> Self {
        Self {
            data: data,
            sign: sign,
        }
    }
}

const MOD: u128 = 1 << BASE_BITS;
const BASE_BITS: u8 = 64;

impl Add for BigInteger {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let new_len = max(self.data.len(), other.data.len());
        let mut target = self.clone().data.to_vec();
        let mut source = other.clone().data.to_vec();
        target.resize(new_len, 0);
        source.resize(new_len, 0);
        let mut round: u64 = 0;
        for i in 0..new_len {
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

impl AddAssign for BigInteger {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Sub for BigInteger {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut target = other.clone();
        {
            let len = target.data.len();
            for i in 0..len {
                target.data[i] = !target.data[i];
            }
            target += ONE.clone();
        }
        let mut res = self + target;
        {
            let len = res.data.len();
            let top = res.data[len - 1];
            if top == 1 {
                res.data.remove(len - 1);
            } else {
                let mut bit_len = BASE_BITS;
                while bit_len > 0 && top & (1 << (bit_len - 1)) == 0 {
                    bit_len -= 1;
                }
                if bit_len == 0 {
                    res.data.remove(len - 1);
                    if len >= 2 {
                        let top2 = res.data[len - 2];
                        let mut bit_len2 = BASE_BITS;
                        while bit_len2 > 0 && top2 & (1 << (bit_len2 - 1)) == 0 {
                            bit_len2 -= 1;
                        }
                        res.data[len - 2] ^= 1 << (bit_len2 - 1);
                    }
                } else {
                    res.data[len - 1] ^= 1 << (bit_len - 1);
                }
            }
        }
        res
    }
}

impl SubAssign for BigInteger {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}
