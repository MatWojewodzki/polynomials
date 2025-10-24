use std::collections::BTreeMap;
use num::Num;
use num::traits::Pow;

mod coefficients;
mod parsing;
mod arithmetic;
pub mod display;

/// Represents a univariate polynomial with real coefficients.
///
/// # Examples
///
/// Create a representation of a polynomial function `Q(x) = 2x^2 - 3x + 2` and calculate
/// the value of its derivative at the point `x = 2`:
/// ```
/// use polynomials::Polynomial;
///
/// let poly = Polynomial::from_coefficients(&vec![2.0, -3.0, 2.0]);
/// let derivative = poly.derivative();
/// assert_eq!("4x - 3", derivative.to_string());
///
/// let value = derivative.evaluate(2.0);
/// assert_eq!(5.0, value);
/// ```
#[derive(PartialEq, Debug, Clone)]
pub struct Polynomial<T> {
    coefficients: BTreeMap<u32, T>,
}

impl<T> Polynomial<T> {
    /// Returns a new polynomial with all coefficients set to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let poly = Polynomial::zero();
    /// assert!(poly.is_zero());
    /// ```
    pub fn zero() -> Polynomial<T> {
        Polynomial {
            coefficients: BTreeMap::new(),
        }
    }

    /// Checks if the polynomial is a zero polynomial.
    ///
    /// # Examples
    ///
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let mut poly = Polynomial::zero();
    /// assert!(poly.is_zero());
    ///
    /// poly.set_coefficient_at(0, 3.0);
    /// assert!(!poly.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.coefficients.is_empty()
    }

    /// Returns the degree of the polynomial.
    ///
    /// Returns `Some(u32)` if the polynomial is not the zero polynomial, otherwise returns `None`.
    ///
    /// # Examples
    ///
    /// Get the degree of a quadratic polynomial:
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let poly = Polynomial::from_coefficients(&vec![2.0, -1.0, 1.0]);
    /// let degree = poly.degree().unwrap();
    /// assert_eq!(2, degree);
    /// ```
    ///
    /// Degree of the zero polynomial is undefined:
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let poly = Polynomial::zero();
    /// assert!(poly.degree().is_none());
    /// ```
    pub fn degree(&self) -> Option<u32> {
        self.coefficients.keys().rev().next().copied()
    }

    /// Sets all coefficients to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let mut poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -10.0, 5.0]);
    /// poly.clear();
    /// assert!(poly.is_zero());
    /// ```
    pub fn clear(&mut self) {
        self.coefficients.clear();
    }
}

impl<T> Polynomial<T>
where
    T: Num + Clone + Pow<T, Output = T>,
    u32: Into<T>,
{
    /// Evaluates the polynomial at a given x using Horner's method.
    ///
    /// # Examples
    ///
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let poly = Polynomial::from_coefficients(&vec![1.0, 1.0, -2.0]);
    /// let value = poly.evaluate(1.0);
    /// assert_eq!(0.0, value);
    pub fn evaluate(&self, x: T) -> T {
        let mut result = T::zero();
        let mut last_power: Option<u32> = None;

        for (power, coefficient) in self.coefficients.iter().rev() {
            if let Some(last_x_power) = last_power {
                let power_diff = last_x_power - *power;
                result = result * x.clone().pow(power_diff.into());
            }

            result = result + coefficient.clone();
            last_power = Some(*power);
        }
        result
    }
}

impl<T> Polynomial<T>
where
    T: Num + Clone,
    u32: Into<T>,
{
    /// Returns the derivative of a polynomial function.
    ///
    /// # Examples
    ///
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let poly = Polynomial::from_coefficients(&vec![1.0, -2.0, 0.0, -10.0]);
    /// let derivative = poly.derivative();
    /// assert_eq!(vec![3.0, -4.0, 0.0], derivative.get_coefficients());
    /// ```
    pub fn derivative(&self) -> Self {
        let mut result = Polynomial::zero();
        for (power, coefficient) in self.coefficients.iter() {

            // Skip the zero-power term to avoid u32 subtraction with overflow
            if *power < 1 {
                continue;
            }
            result.set_coefficient_at(*power - 1, coefficient.clone() * (*power).into());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn is_zero_works() {
        let mut poly = Polynomial::zero();
        assert!(poly.is_zero());
        poly.set_coefficient_at(0, 3.0);
        assert!(!poly.is_zero());
    }

    #[test]
    fn degree_works() {
        let mut poly = Polynomial::from_coefficients(&vec![-2.0]);
        assert_eq!(poly.degree(), Some(0));

        poly.set_coefficient_at(2, 3.0);
        assert_eq!(poly.degree(), Some(2));

        poly.set_coefficient_at(1, 2.0);
        assert_eq!(poly.degree(), Some(2));

        poly.set_coefficient_at(5, 0.0);
        assert_eq!(poly.degree(), Some(2));

        poly.set_coefficient_at(1234, 1.0);
        assert_eq!(poly.degree(), Some(1234));
    }

    #[test]
    fn degree_handles_zero_polynomial() {
        let poly: Polynomial<f64> = Polynomial::zero();
        assert_eq!(poly.degree(), None);
    }

    #[test]
    fn polynomial_clear() {
        let mut poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        poly.clear();
        assert!(poly.is_zero());
    }

    #[test]
    fn polynomial_equality() {
        let poly1 = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly2 = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        assert_eq!(poly1, poly2);
    }

    #[test]
    fn evaluate_works() {
        let poly = Polynomial::from_coefficients(&vec![3.0, 2.0, 0.0, -3.0]);
        assert_eq!(-19.0, poly.evaluate(-2.0));
    }

    #[test]
    fn derivative_works() {
        let poly = Polynomial::from_coefficients(&vec![3.0, 2.0, 0.0, -3.0]);
        let derivative = poly.derivative();
        assert_eq!(vec![9.0, 4.0, 0.0], derivative.get_coefficients());
    }
}