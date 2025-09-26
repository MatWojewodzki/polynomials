//! A library for working with univariate polynomials.
//!
//! # Examples
//! 
//! ## Creating a Polynomial
//! 
//! Instantiating a polynomial as the zero polynomial and setting the coefficients afterwards:
//! 
//! ```
//! let mut poly = polynomials::Polynomial::zero();
//! 
//! poly.set_coefficient_at(3, 1.0);
//! poly.set_coefficient_at(2, -1.0);
//! poly.set_coefficient_at(0, -2.0);
//! 
//! assert_eq!("x^3 - x^2 - 2", poly.to_string());
//! ```
//! 
//! Creating a polynomial from a vector of coefficients:
//! ```
//! let poly = polynomials::Polynomial::from_coefficients(&vec![1.0, 1.0, -2.0]);
//! assert_eq!("x^2 + x - 2", poly.to_string());
//! ```
//! 
//! Creating a polynomial from a string:
//! 
//! ```
//! let poly = polynomials::Polynomial::from_string("2x^2 + 3x - 1").unwrap();
//! assert_eq!("2x^2 + 3x - 1", poly.to_string());
//! ```
//! 
//! ## Evaluating a Polynomial at a given x
//! 
//! ```
//! let poly = polynomials::Polynomial::from_coefficients(&vec![2.0, -2.0, 0.0, -1.0]);
//! let value = poly.evaluate(-2.0);
//! assert_eq!(-25.0, value);
//! ```
//! 
//! ## Displaying a polynomial
//! 
//! ```
//! let poly = polynomials::Polynomial::from_coefficients(&vec![2.0, -2.0, 0.0, -1.0]);
//! println!("Q(x) = {}", poly);
//! assert_eq!("2x^3 - 2x^2 - 1", poly.to_string());
//! ```
//! 
//! ## Performing arithmetic operations on polynomials
//! 
//! ```
//! use polynomials::Polynomial;
//! 
//! // addition
//! let term1 = Polynomial::from_coefficients(&vec![2.0, -2.0, 0.0, -1.0]);
//! let term2 = Polynomial::from_coefficients(&vec![1.0, 1.0, -2.0]);
//! let sum = Polynomial::from_coefficients(&vec![2.0, -1.0, 1.0, -3.0]);
//! assert_eq!(sum, term1 + term2);
//! 
//! let poly = Polynomial::from_coefficients(&vec![1.0, 0.0]);
//! let poly_plus_five = Polynomial::from_coefficients(&vec![1.0, 5.0]);
//! assert_eq!(poly_plus_five, poly + 5.0);
//!
//! // subtraction
//! let term1 = Polynomial::from_coefficients(&vec![1.0, 1.0, -2.0]);
//! let term2 = Polynomial::from_coefficients(&vec![2.0, -2.0, 0.0, -1.0]);
//! let difference = Polynomial::from_coefficients(&vec![-2.0, 3.0, 1.0, -1.0]);
//! assert_eq!(difference, term1 - term2);
//! 
//! let poly = Polynomial::from_coefficients(&vec![1.0, 0.0]);
//! let poly_minus_five = Polynomial::from_coefficients(&vec![1.0, -5.0]);
//! assert_eq!(poly_minus_five, poly - 5.0);
//! 
//! // multiplication
//! let factor1 = Polynomial::from_coefficients(&vec![1.0, -1.0]);
//! let factor2 = Polynomial::from_coefficients(&vec![1.0, 2.0]);
//! let product = Polynomial::from_coefficients(&vec![1.0, 1.0, -2.0]);
//! assert_eq!(product, factor1 * factor2);
//! 
//! let poly = Polynomial::from_coefficients(&vec![1.0, -2.0]);
//! let poly_times_five = Polynomial::from_coefficients(&vec![5.0, -10.0]);
//! assert_eq!(poly_times_five, poly * 5.0);
//! 
//! // division
//! let poly = Polynomial::from_coefficients(&vec![2.0, 0.0, -4.0]);
//! let poly_divided_by_two = Polynomial::from_coefficients(&vec![1.0, 0.0, -2.0]);
//! assert_eq!(poly_divided_by_two, poly / 2.0);
//!
//! // negation
//! let poly = Polynomial::from_coefficients(&vec![1.0, 1.0, -2.0]);
//! let negated = Polynomial::from_coefficients(&vec![-1.0, -1.0, 2.0]);
//! assert_eq!(negated, -poly);
//! ```

mod polynomial;

pub use polynomial::Polynomial;