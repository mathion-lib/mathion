<img src="./mathion-logo.png" width="20%" height="20%" title="test" alt="mathion-logo"></img>
# mathion (math + ion) - computational mathematics library
[![MIT][s1]][li]

[s1]: https://img.shields.io/badge/License-MIT-blue.svg

[li]: LICENSE
! Warning ! This library must run in the Rust `nightly`
# TODO
## Fast-Fourier-Transform by using [Cooley-Tukey algorithm](https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm)
- [x] Number theoretic
- [ ] Normal
## Ring
### Integer (Currently, Big Integer)
- [x] Addition, Subtraction
- [ ] Multiplication
### Matrix
- [ ] Addition, Subtraction
- [ ] Multiplication
- [ ] Inversion
## Field
### Galois Field (Small, by using native integer (`i64`))
- [x] Addition, Subtraction
- [x] Multiplication, Division
### Polynomial (Using galois field (small))
- [x] Addition, Subtraction
- [x] Multiplication, Division
