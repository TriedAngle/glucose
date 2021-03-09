##The current schedule is to have 90% implemented by July 2021
###(But: feature list may be subjected to change any time)

# Algebra
## Linear Algebra
- [x] Vectors
- [x] Matrices
- [x] Dynamic Vectors
- [x] Dynamic Matrices
- [X] basic "specialisation" (>1000x performance boost) by wrapper types for N < 9
- [x] correct algebraic structure traits (fructose library)
- [ ] migrate glucose to fructose
- [ ] 2D Bivector (partially implement already)
- [ ] 3D Bivector
- [ ] 2D Rotor (partially implement already)
- [ ] 3D Rotor
- [ ] Abstract Rotors over N-dimensions (and specialize 2D & 3D)
- [ ] Quaternions
- [ ] Matrix Determinant
- [ ] LU Decomposition

# Number Theory
- [x] prime factorization
- [x] coprimes
- [x] euclidean algorithm
- [x] extended euclidean algorithm
- [ ] Stirling numbers of the first kind
- [ ] Stirling numbers of the second kind
- [ ] DPLL
- [ ] Linear Resolution
- [ ] generify everything

# Abstract over Types
- [x] Basic Numeric Traits
- [x] Trigonometry Trait 
- [x] Power Trait
- [ ] Add Complex Number
- [ ] impl ComplexField for Real and Complex Numbers

# Group Theory
- [ ] migrate current algos and types to fructose
- [x] Types (Additive, Multiplicative, Multiplicative with Modulo)
- [x] group size
- [x] whole group
- [x] orders (multiplicative none modulo needs a fix)
- [x] producers (algorithm needs a fix for numbers bigger 54)
- [ ] cyclic groups with fructose
- [ ] control elements

# Calculus
- [ ] some more traits
- [ ] implement functions with fructose traits
- [ ] integrals and derivatives
- [ ] solvers

# Statistics (requires calculus)
- [ ] Traits
- [ ] Types

# Bioinformatics (requires statistics)
- [ ] Types e.g. Alphabets etc.
- [ ] Derivative of Boyer-Moore
- [ ] Derivative of KMP

# Other
## quality
- [ ] more tests (at least 90% of the methods once)
- [ ] documentation (at least 90% of all exposed apis)
- [ ] decide whether or not this crate should be split up (maybe take out the traits?)
- [ ] semi-good looking web-interface(with at least 75% covered functionality)

## library implementations:
- [x] bytemuck
- [ ] serde
- [ ] mint

# Future (until the end of the year)
- [ ] Tools for Bioinformatics (maybe in a separate crate?)
- [ ] SIMD optimizations
- [ ] BLAS