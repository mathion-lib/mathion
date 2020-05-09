/*
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

#![feature(vec_into_raw_parts)]
#![feature(const_fn)]

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
        let f = IntPolynomial::from(vec![1, 2, 1]);
        let g = IntPolynomial::from(vec![1, 1]);
        let q = f.clone() / g.clone();
        let rem = f.clone() % g.clone();
        assert_eq!(q, IntPolynomial::from(vec![1, 1]));
        assert_eq!(rem, IntPolynomial::from(vec![0]));
        assert_eq!(
            IntPolynomial::from(vec![0, 0, 1]),
            IntPolynomial::from(vec![0, 1]) * IntPolynomial::from(vec![0, 1])
        );
        assert_eq!(
            BigInteger::new(vec![12351838, 32], false) - BigInteger::new(vec![32, 32], false),
            BigInteger::new(vec![12351806, 0], false)
        );
    }
}
