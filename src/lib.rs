/*
    Author : quickn (quickn.ga)
    Email  : quickwshell@gmail.com
*/

#![feature(vec_into_raw_parts)]

mod fft;
mod matrix;
mod polynomial;
mod prelude;
mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::polynomial::IntPolynomial;
        use crate::types::BigInteger;
        let f = IntPolynomial::new(vec![1, 2, 1]);
        let g = IntPolynomial::new(vec![1, 1]);
        let q = f.clone() / g.clone();
        let rem = f.clone() % g.clone();
        //assert_eq!(q, IntPolynomial::new(vec![13,1,2]));
        //assert_eq!(rem, IntPolynomial::new(vec![47]));
        assert_eq!(
            IntPolynomial::new(vec![0, 0, 1]),
            IntPolynomial::new(vec![0, 1]) * IntPolynomial::new(vec![0, 1])
        );
        assert_eq!(
            BigInteger::new(vec![12351838, 32], false) - BigInteger::new(vec![32, 32], false),
            BigInteger::new(vec![12351806, 0], false)
        );
    }
}
