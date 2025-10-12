use std::ops::{Div, DivAssign, Rem, RemAssign};
use super::Polynomial;

pub struct PolynomialDivisionResult {
    pub quotient: Polynomial,
    pub remainder: Polynomial
}

struct Term {
    coefficient: f64,
    power: u32
}

/// Returns a leading term of a [`Polynomial`].
fn leading_term(poly: &Polynomial) -> Term {
    let degree = poly.degree().unwrap();
    Term {
        coefficient: poly.get_coefficient_at(degree),
        power: degree
    }
}

/// Returns a quotient of two terms as a [`Polynomial`].
fn divide_terms(term1: Term, term2: Term) -> Polynomial {
    let mut quotient = Polynomial::zero();
    quotient.set_coefficient_at(
        term1.power - term2.power,
        term1.coefficient / term2.coefficient
    );
    quotient
}

/// Polynomial division algorithm described here
/// https://en.wikipedia.org/wiki/Polynomial_long_division#Pseudocode
///
/// After a function invocation, a quotient is returned and the numerator becomes a
/// remainder of the division.
fn divide_in_place(numerator: &mut Polynomial, denominator: &Polynomial) -> Polynomial {
    if denominator.is_zero() {
        panic!("Cannot divide by the zero polynomial.");
    }

    let mut quotient = Polynomial::zero();
    let remainder = numerator;

    while !remainder.is_zero() && remainder.degree().unwrap() >= denominator.degree().unwrap() {
        let next_quotient_term = divide_terms(
            leading_term(&remainder), leading_term(denominator)
        );
        quotient += &next_quotient_term;
        *remainder -= &(next_quotient_term * denominator);
    }

    quotient
}

fn divide_by_scalar_in_place(poly: &mut Polynomial, scalar: f64) {
    if scalar == 0.0 {
        panic!("Cannot divide by zero.");
    }
    for (_, coefficient) in poly.coefficients.iter_mut() {
        *coefficient /= scalar;
    }
}

impl Div<&Self> for Polynomial {
    type Output = PolynomialDivisionResult;

    fn div(mut self, rhs: &Self) -> Self::Output {
        let quotient = divide_in_place(&mut self, rhs);
        PolynomialDivisionResult {
            quotient,
            remainder: self
        }
    }
}

impl Div<f64> for Polynomial {
    type Output = Polynomial;

    fn div(mut self, rhs: f64) -> Self::Output {
        divide_by_scalar_in_place(&mut self, rhs);
        self
    }
}

impl Div<i32> for Polynomial {
    type Output = Polynomial;

    fn div(mut self, rhs: i32) -> Self::Output {
        divide_by_scalar_in_place(&mut self, rhs as f64);
        self
    }
}

impl DivAssign<&Self> for Polynomial {
    fn div_assign(&mut self, rhs: &Self) {
        *self = divide_in_place(self, rhs);
    }
}

impl DivAssign<f64> for Polynomial {
    fn div_assign(&mut self, rhs: f64) {
        divide_by_scalar_in_place(self, rhs);
    }
}

impl DivAssign<i32> for Polynomial {
    fn div_assign(&mut self, rhs: i32) {
        divide_by_scalar_in_place(self, rhs as f64);
    }
}

impl Rem<&Self> for Polynomial {
    type Output = Polynomial;

    fn rem(mut self, rhs: &Self) -> Self::Output {
        divide_in_place(&mut self, rhs);
        self
    }
}

impl RemAssign<&Self> for Polynomial {
    fn rem_assign(&mut self, rhs: &Self) {
        divide_in_place(self, rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn div() {
        let numerator = Polynomial::from_coefficients(&vec![-4.0, 12.0, -21.0, 19.0, 0.0]);
        let denominator = Polynomial::from_coefficients(&vec![2.0, -3.0, 5.0]);
        let div_result  = numerator / &denominator;
        assert_eq!(vec![-2.0, 3.0, -1.0], div_result.quotient.get_coefficients());
        assert_eq!(vec![1.0, 5.0], div_result.remainder.get_coefficients());
    }

    #[test]
    fn div_float() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly_divided_by_two = poly / 2.0;
        assert_eq!(vec![0.5, 1.0, -1.5], poly_divided_by_two.get_coefficients());
    }

    #[test]
    fn div_int() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly_divided_by_two = poly / 2;
        assert_eq!(vec![0.5, 1.0, -1.5], poly_divided_by_two.get_coefficients());
    }

    #[test]
    fn div_assign() {
        let mut poly = Polynomial::from_coefficients(&vec![1.0, 4.0, -1.0, -3.0]);
        let divisor = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        poly /= &divisor;
        assert_eq!(vec![1.0, 2.0], poly.get_coefficients());
    }

    #[test]
    fn div_assign_float() {
        let mut poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        poly /= 2.0;
        assert_eq!(vec![0.5, 1.0, -1.5], poly.get_coefficients());
    }

    #[test]
    fn div_assign_int() {
        let mut poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        poly /= 2;
        assert_eq!(vec![0.5, 1.0, -1.5], poly.get_coefficients());
    }

    #[test]
    fn rem() {
        let numerator = Polynomial::from_coefficients(&vec![1.0, 4.0, -1.0, -3.0]);
        let denominator = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let remainder = numerator % &denominator;
        assert_eq!(vec![-2.0, 3.0], remainder.get_coefficients());
    }

    #[test]
    fn rem_assign() {
        let mut poly = Polynomial::from_coefficients(&vec![1.0, 4.0, -1.0, -3.0]);
        let divisor = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        poly %= &divisor;
        assert_eq!(vec![-2.0, 3.0], poly.get_coefficients());
    }

    #[test]
    #[should_panic(expected = "Cannot divide")]
    fn div_by_zero_polynomial() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let _ = poly / &Polynomial::zero();
    }

    #[test]
    #[should_panic(expected = "Cannot divide")]
    fn div_by_zero_float() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let _ = poly / 0.0;
    }

    #[test]
    #[should_panic(expected = "Cannot divide")]
    fn div_by_zero_int() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let _ = poly / 0;
    }
}