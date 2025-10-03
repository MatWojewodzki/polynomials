//! A library for working with univariate polynomials.
//!
//! The library exposes the [`Polynomial`] struct, which provides all the functionality.
//!
//! # Examples
//! 
//! ## Creating a Polynomial
//! 
//! Instantiating a polynomial as the zero polynomial and setting the coefficients afterwards:
//! 
//! ```
//! use polynomials::Polynomial;
//! 
//! let mut poly = Polynomial::zero();
//! poly.set_coefficient_at(3, 1.0);
//! poly.set_coefficient_at(2, -1.0);
//! poly.set_coefficient_at(0, -2.0);
//! 
//! assert_eq!(vec![1.0, -1.0, 0.0, -2.0], poly.get_coefficients());
//! ```
//! 
//! Creating a polynomial from a vector of coefficients:
//! ```
//! use polynomials::Polynomial;
//! 
//! let poly = Polynomial::from_coefficients(&vec![1.0, 1.0, -2.0]);
//! assert_eq!(vec![1.0, 1.0, -2.0], poly.get_coefficients());
//! ```
//! 
//! Creating a polynomial from a string:
//! 
//! ```
//! use polynomials::Polynomial;
//! 
//! let poly = Polynomial::from_string("2x^2 + 3x - 1").unwrap();
//! assert_eq!(vec![2.0, 3.0, -1.0], poly.get_coefficients());
//! ```
//! 
//! ## Evaluating a Polynomial at a given x
//! 
//! ```
//! use polynomials::Polynomial;
//! 
//! let poly = Polynomial::from_coefficients(&vec![2.0, -2.0, 0.0, -1.0]);
//! let value = poly.evaluate(-2.0);
//! assert_eq!(-25.0, value);
//! ```
//! 
//! ## Displaying a polynomial
//! 
//! ```
//! use polynomials::Polynomial;
//! 
//! let poly = Polynomial::from_coefficients(&vec![2.0, -2.0, 0.0, -1.0]);
//! let poly_string = poly.to_string();
//! println!("Q(x) = {}", poly_string);
//! assert_eq!("2x^3 - 2x^2 - 1", poly_string);
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
pub use polynomial::display::PolynomialFormat;