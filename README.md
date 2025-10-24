# polynomials

Rust library crate for working with univariate polynomials.

## TODO list:
- add tests for the `Polynomial.format_with()` method to check if it works properly with `Complex` and `Rational` numbers
- adjust the `Polynomial::from_str()` function to handle `Complex` numbers properly with curly braces
- write proper tests for the `Polynomial::from_str()` function and fix possible bugs
- use `std::sync::LazyLock` to prevent the regex from being recompiled on every call to `Polynomial::from_str()`
- update the documentation to reflect the fact that the `Polynomial` struct is now generic
- split the documentation of arithmetic operations into separate code blocks in the crate documentation
- publish the create on crates.io
- add more content to this README file
- provide implementations of arithmetic operations for other combinations of references and owned values