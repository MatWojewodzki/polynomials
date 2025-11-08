# polynomials

Rust library crate for working with univariate polynomials.

## TODO list:
- rewrite `Polynomial::from_str()`
    - drop usage of the `regex` crate
    - restore the strictness of parsing
    - add support for indeterminate other than 'x'
    - consider support for polynomials in a factored form (e.g. `3x(x- 1)(x + 2)`)
- update the documentation to reflect the fact that the `Polynomial` struct is now generic
- split the documentation of arithmetic operations into separate code blocks in the crate documentation
- publish the crate on crates.io
- add more content to this README file
- provide implementations of arithmetic operations for other combinations of references and owned values