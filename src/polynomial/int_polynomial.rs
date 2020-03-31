/*
    Author : quickn (quickn.ga)
    Email  : quickwshell@gmail.com
*/

// Dependence on small_ntt
use crate::fft::small_ntt::{MAX, P, fft};

use std::cmp::max;
use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign, Rem, RemAssign};

// Polynomial representation in Z[x]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntPolynomial {
    data: Vec<i64>,
    deg: usize,
}

impl IntPolynomial {
    pub fn new(data: Vec<i64>) -> Self {
        let mut deg = 0;
        if data.len() != 0 {
            deg = data.len()-1;
            while data[deg] == 0 { if deg == 0 { break; } deg -= 1; }
        }
        Self {
            data: data.get(0..=deg).unwrap().to_vec(),
            deg: deg,
        }
    }

    pub fn deg(&self) -> usize {
        self.deg
    }

    pub fn modulo(&self, x: usize) -> Self {
        if self.deg() < x {
            self.clone()
        } else {
            Self {
                data: self.data.get(0..x).unwrap().to_vec(),
                deg: (x-1),
            }
        }
    }

    pub fn reciprocal(&self) -> Self {
        let mut data = self.data.clone();
        data.reverse();
        Self {
            data: data,
            deg: self.deg(),
        }
    }

    fn rshift(&self, k: usize) -> Self {
        let mut res = Vec::new();
        res.resize(k, 0);
        let mut t = self.data.clone();
        res.append(&mut t);
        Self::new(res)
    }

    fn lshift(&self, k: usize) -> Self {
        Self::new(self.data.get(k..=self.deg()).unwrap().to_vec())
    }
}

impl Add for IntPolynomial {
    type Output = IntPolynomial;
    fn add(self, other: Self) -> Self {
        let mut origin = self.data.clone();
        let mut source = other.data.clone();
        let new_deg = max(self.deg(), other.deg())+1;
        origin.resize(new_deg, 0);
        source.resize(new_deg, 0);
        for i in 0..new_deg {
            origin[i] += source[i];
        }
        Self::new(origin)
    }
}

impl AddAssign for IntPolynomial {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Sub for IntPolynomial {
    type Output = IntPolynomial;
    fn sub(self, other: Self) -> Self {
        let mut origin = self.data.clone();
        let mut source = other.data.clone();
        let new_deg = max(self.deg(), other.deg())+1;
        origin.resize(new_deg, 0);
        source.resize(new_deg, 0);
        for i in 0..new_deg {
            origin[i] -= source[i];
        }
        Self::new(origin)
    }
}

impl SubAssign for IntPolynomial {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl Neg for IntPolynomial {
    type Output = IntPolynomial;
    fn neg(self) -> Self {
        let mut origin = self.data.clone();
        for i in 0..self.deg() {
            origin[i] = -origin[i];
        }
        Self {
            data: origin,
            deg: self.deg(),
        }
    }
}

impl Mul for IntPolynomial {
    type Output = IntPolynomial;
    fn mul(self, other: Self) -> Self {
        let (mut p, mut q) = (self.data.clone(), other.data.clone());
        let t = max(self.deg(), other.deg())+1;
        let mut new_sz = 1;
        let mut shift = 0;
        while new_sz <= (t<<1) {
            new_sz <<= 1;
            shift += 1;
        }
        p.resize(new_sz, 0);
        q.resize(new_sz, 0);
        fft(&mut p, shift, false);
        fft(&mut q, shift, false);
        for i in 0..new_sz {
            p[i] = (p[i]*q[i]) % P;
        }
        fft(&mut p, shift, true);
        for i in 0..new_sz {
            if p[i] < 0 {
                p[i] += P as i64;
                p[i] %= P as i64;
            }
            if p[i] > (MAX as i64) {
                p[i] = p[i] - (P as i64);
            }
        }
        Self::new(p)
    }
}

impl Mul<IntPolynomial> for i64 {
    type Output = IntPolynomial;
    fn mul(self, other: Self::Output) -> Self::Output {
        if self == 0 {
            Self::Output::new(vec![0])
        } else {
            let mut data = other.data.clone();
            for i in 0..=other.deg() {
                data[i] *= self;
            }
            Self::Output {
                data: data,
                deg: other.deg(),
            }
        }
    }
}

impl Mul<i64> for IntPolynomial {
    type Output = IntPolynomial;
    fn mul(self, other: i64) -> Self {
        if other == 0 {
            Self::new(vec![0])
        } else {
            let mut data = self.data.clone();
            for i in 0..=self.deg() {
                data[i] *= other;
            }
            Self {
                data: data,
                deg: self.deg(),
            }
        }
    }
}

impl MulAssign for IntPolynomial {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other
    }
}

impl Div for IntPolynomial {
    type Output = IntPolynomial;
    fn div(self, other: Self) -> Self {
        if self.deg() < other.deg() {
            self
        } else {
            assert_eq!(other.data.last().unwrap().abs(), 1);
            let mut rev_f = self.reciprocal();
            let rev_g = other.reciprocal();
            let mod_t = rev_f.deg() - rev_g.deg() + 1;
            let mut i = 1;
            while i < mod_t {
                i <<= 1;
            }
            rev_f = rev_f.rshift(i-mod_t);
            let mut h = Self::new(vec![*rev_g.data.first().unwrap()]);
            let mut j = 2;
            while j <= i {
                h = (2*h.clone()-rev_g.modulo(j)*h.clone()*h.clone()).modulo(j);
                j <<= 1;
            }
            (rev_f*h).modulo(i).lshift(i-mod_t).reciprocal()
        }
    }
}

impl DivAssign for IntPolynomial {
    fn div_assign(&mut self, other: Self) {
        *self = self.clone() / other
    }
}

impl Rem for IntPolynomial {
    type Output = IntPolynomial;
    fn rem(self, other: Self) -> Self {
        self.clone() - ((self.clone() / other.clone()) * other.clone())
    }
}

impl RemAssign for IntPolynomial {
    fn rem_assign(&mut self, other: Self) {
        *self = self.clone() % other
    }
}