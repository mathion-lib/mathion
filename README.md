# The mathion - computational mathematics library
! Warning ! This library must run in the Rust `nightly`
# TODO
- Fast-Fourier-Transform by using [Cooley-Tukey algorithm](https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm)
  - Number theoretic [x]
  - Normal [ ]
- Ring
  - Integer (Currently, Big Integer)
    - Addition, Subtraction [x]
    - Multiplication [ ]
  - Matrix
    - Addition, Subtraction [ ]
    - Multiplication []
    - Inversion [ ]
- Field
  - Galois Field (Small, by using native integer (`i64`))
    - Addition, Subtraction [x]
    - Multiplication, Division [x]
  - Polynomial (Using galois field (small))
    - Addition, Subtraction [x]
    - Multiplication, Division [x]