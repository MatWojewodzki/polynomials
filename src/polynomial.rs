use std::collections::BTreeMap;
mod coefficients;
mod parsing;
mod arithmetic;
mod display;

#[derive(PartialEq, Debug)]
pub struct Polynomial {
    coefficients: BTreeMap<u32, f64>,
}

impl Polynomial {
    pub fn zero() -> Polynomial {
        Polynomial {
            coefficients: BTreeMap::new(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.coefficients.is_empty()
    }

    pub fn degree(&self) -> Option<u32> {
        self.coefficients.keys().rev().next().copied()
    }

    pub fn clear(&mut self) {
        self.coefficients.clear();
    }

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
    pub fn evaluate(&self, x: f64) -> f64 {
        let mut result = 0.0;
        let mut last_power: Option<u32> = None;

        for (power, coefficient) in self.coefficients.iter().rev() {
            if let Some(last_x_power) = last_power {
                let power_diff = last_x_power - *power;
                result *= x.powi(power_diff as i32);
            }

            result += coefficient;
            last_power = Some(*power);
        }
        result
    }

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
            result.set_coefficient_at(*power - 1, *coefficient * (*power as f64));
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
        let poly = Polynomial::zero();
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