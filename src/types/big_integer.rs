/*
    Author : quickn (quickn.ga)
    Email  : quickwshell@gmail.com
*/

use std::cmp::max;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use crate::polynomial::IntPolynomial;

#[derive(Copy, Clone)]
struct BigInteger<'a> {
    data: &'a [bool],
    // false : positive, true : negative
    sign: bool,
}

impl<'a> BigInteger<'a> {
    fn new(data: Vec<bool>, sign: bool) -> Self {
        let d = data.clone().as_ptr();
        Self {
            data: unsafe { std::slice::from_raw_parts(d, data.len()) },
            sign: sign,
        }
    }

    fn from_i64(n: i64) -> Self {
        let mut data: Vec<bool> = Vec::new();
        let sign = if n < 0 { true } else { false };
        let mut k = n;
        let mut sz = 0;
        while k > 0 {
            if k & 1 == 1 {
                data.push(true);
            } else {
                data.push(false);
            }
            k >>= 1;
            sz += 1;
        }
        Self::new(data.clone(), sign)
    }

    fn reverse(&mut self) {
        let mut new_data = self.data.clone().to_vec();
        new_data.reverse();
        self.data = unsafe { std::slice::from_raw_parts(new_data.as_ptr(), new_data.len()) };
    }
}

impl Add for BigInteger<'_> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let new_sz = max(self.data.len(), other.data.len());
        let mut target = self.clone().data.to_vec();
        let mut source = other.clone().data.to_vec();
        target.resize(new_sz, false);
        source.resize(new_sz, false);
        let mut round = false;
        for i in 0..new_sz {
            let mut tmp = target[i];
            let mut new_round = false;
            target[i] ^= source[i];
            if tmp & source[i] {
                new_round = true;
            }
            tmp = target[i];
            target[i] ^= round;
            if tmp ^ round {
                new_round = true;
            }
            round = new_round;
        }
        if round {
            target.push(true);
        }
        Self::new(target.clone(), self.sign)
    }
}

impl AddAssign for BigInteger<'_> {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

// Not implemented yet

/*impl Sub for BigInteger<'_> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let new_sz = max(self.data.len(), other.data.len());
        let mut target = self.clone().data.to_vec();
        let mut source = other.clone().data.to_vec();
        target.resize(new_sz, false);
        source.resize(new_sz, false);
        let mut round = false;
        for i in 0..new_sz {
            let mut tmp = target[i];
            let mut new_round = false;
            target[i] ^= source[i];
            if tmp & source[i] {
                new_round = true;
            }
            tmp = target[i];
            target[i] ^= round;
            if tmp ^ round {
                new_round = true;
            }
            round = new_round;
        }
        if round {
            target.push(true);
        }
        Self::new(target.clone(), self.sign)
    }
}

impl SubAssign for BigInteger<'_> {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}*/