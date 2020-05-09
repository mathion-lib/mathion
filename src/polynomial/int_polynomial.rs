/*
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

// Dependence on small_ntt
use crate::fft::{do_fft, FFTType};
use crate::types::field::{self, GaloisField as Field};

use std::cmp::max;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

// Polynomial representation in Z[x]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntPolynomial {
    data: Vec<Field>,
    deg: usize,
}

impl IntPolynomial {
    pub fn new(data: Vec<Field>) -> Self {
        let mut deg = 0;
        if data.len() != 0 {
            deg = data.len() - 1;
            while data[deg] == field::ZERO {
                if deg == 0 {
                    break;
                }
                deg -= 1;
            }
        }
        Self {
            data: data.get(0..=deg).unwrap().to_vec(),
            deg: deg,
        }
    }

    pub fn from(data: Vec<i64>) -> Self {
        let mut deg = 0;
        if data.len() != 0 {
            deg = data.len() - 1;
            while data[deg] == 0 {
                if deg == 0 {
                    break;
                }
                deg -= 1;
            }
        }
        Self {
            data: data
                .get(0..=deg)
                .unwrap()
                .iter()
                .map(|&d| Field::from(d))
                .collect(),
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
                deg: (x - 1),
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
        res.resize(k, field::ZERO);
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
        let new_deg = max(self.deg(), other.deg()) + 1;
        origin.resize(new_deg, field::ZERO);
        source.resize(new_deg, field::ZERO);
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
        let new_deg = max(self.deg(), other.deg()) + 1;
        origin.resize(new_deg, field::ZERO);
        source.resize(new_deg, field::ZERO);
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
        let t = max(self.deg(), other.deg()) + 1;
        let mut new_sz = 1;
        while new_sz <= (t << 1) {
            new_sz <<= 1;
        }
        p.resize(new_sz, field::ZERO);
        q.resize(new_sz, field::ZERO);
        do_fft::<Field>(&mut p, FFTType::Straight);
        do_fft::<Field>(&mut q, FFTType::Straight);
        //dbg!(p.clone(), q.clone());
        for i in 0..new_sz {
            p[i] *= q[i];
        }
        do_fft::<Field>(&mut p, FFTType::Inverse);
        Self::new(p)
    }
}

impl Mul<IntPolynomial> for i64 {
    type Output = IntPolynomial;
    fn mul(self, other: Self::Output) -> Self::Output {
        if self == 0 {
            Self::Output::new(vec![field::ZERO])
        } else {
            let mut data = other.data.clone();
            for i in 0..=other.deg() {
                data[i] *= Field::from(self);
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
            Self::new(vec![field::ZERO])
        } else {
            let mut data = self.data.clone();
            for i in 0..=self.deg() {
                data[i] *= Field::from(other);
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
            assert_eq!(other.data.last().unwrap().abs(), field::ONE);
            let mut rev_f = self.reciprocal();
            let rev_g = other.reciprocal();
            let mod_t = rev_f.deg() - rev_g.deg() + 1;
            let mut i = 1;
            while i < mod_t {
                i <<= 1;
            }
            rev_f = rev_f.rshift(i - mod_t);
            let mut h = Self::new(vec![*rev_g.data.first().unwrap()]);
            let mut j = 2;
            while j <= i {
                h = (2 * h.clone() - rev_g.modulo(j) * h.clone() * h.clone()).modulo(j);
                j <<= 1;
            }
            (rev_f * h).modulo(i).lshift(i - mod_t).reciprocal()
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
