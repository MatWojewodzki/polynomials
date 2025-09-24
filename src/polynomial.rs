use std::collections::BTreeMap;
mod coefficients;
mod parsing;
mod arithmetic;
mod display;

#[derive(PartialEq)]
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

    pub fn from_coefficients(coefficients: &Vec<f64>) -> Polynomial {
        let mut poly = Polynomial::zero();
        for (power, coefficient) in (0..coefficients.len()).rev().zip(coefficients.iter()) {
            poly.set_coefficient_at(power as u32, *coefficient);
        }
        poly
    }

    /// Evaluate the polynomial at a given x using Horner's method.
    ///
    /// # Examples
    ///
    /// ```
    /// let poly = polynomials::Polynomial::from_coefficients(&vec![1.0, 1.0, -2.0]);
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
    fn from_coefficients_works_correctly() {
        let poly = Polynomial::from_coefficients(&vec![2.0, 0.0, 2.0, -3.0]);
        assert_eq!(poly.get_coefficient_at(3), 2.0);
        assert_eq!(poly.get_coefficient_at(2), 0.0);
        assert_eq!(poly.get_coefficient_at(1), 2.0);
        assert_eq!(poly.get_coefficient_at(0), -3.0);
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
        assert!(poly1 == poly2);
    }

    #[test]
    fn evaluate_works() {
        let poly = Polynomial::from_coefficients(&vec![3.0, 2.0, 0.0, -3.0]);
        assert_eq!(poly.evaluate(-2.0), -19.0);
    }
}