
Features
===

- [x] Addition/Subtraction
  - [ ] Unary minus
- [x] Multiplication/Division
  - [x] Assume multiplication when two operands are adjacent e.g. (3)(4) = 3(4) = 12, 2pi = tau
- [ ] Braces
  - [x] Basic
  - [x] Nested
  - [ ] Match checking
  - [ ] Completion/Correction
- [x] Powers
  - [x] Real
  - [x] Complex (Uses num crate)
- [ ] Roots
- [ ] Functions
  - [ ] Trigonometry
- [x] Constants (in f64 context)
  - [x] pi
  - [x] [tau](https://tauday.com/)
  - [x] e
- [ ] Multidimensional
  - [x] Complex numbers
  - [ ] Vectors
  - [ ] Matrices

- [ ] Expression entry
  - [x] Ignore whitespace
  - [ ] Show corrected brackets

- [ ] CLI Terminal mode
  - [x] Basic
  - [x] History
  - [x] Clear history
  - [x] Help
  - [ ] Colors
  - [ ] Completion
  - [ ] Memory
    - [ ] Variables
    - [ ] User functions

TODO
==

 - [ ] After calc, ensure stack is exactly 1
 - [ ] Errors
   - [x] Token parse error
   - [ ] Mismatched brackets
   - [ ] Stack size

Errors
===

 - [ ] Complex:
   - [ ] (0-1)^0.5 = 1 (should be i)
