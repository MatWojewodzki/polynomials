//! Module containing methods for manipulating the coefficients of a polynomial.
use super::Polynomial;

impl Polynomial {
    /// Sets the coefficient in the term with the indeterminate raised to the given power.
    ///
    /// # Examples
    ///
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let mut poly = Polynomial::zero();
    /// poly.set_coefficient_at(0, 3.0);
    /// poly.set_coefficient_at(3, -2.0);
    /// assert_eq!(vec![-2.0, 0.0, 0.0, 3.0], poly.get_coefficients());
    /// ```
    pub fn set_coefficient_at(&mut self, power: u32, coefficient: f64) {
        if coefficient == 0.0 {
            self.coefficients.remove(&power);
            return;
        }
        self.coefficients.insert(power, coefficient);
    }

    /// Returns the coefficient from the term with the indeterminate raised to the given power.
    ///
    /// # Examples
    ///
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let poly = Polynomial::from_coefficients(&vec![-1.0, 0.0, 3.0]);
    /// assert_eq!(-1.0, poly.get_coefficient_at(2));
    /// assert_eq!(0.0, poly.get_coefficient_at(1));
    /// assert_eq!(3.0, poly.get_coefficient_at(0));
    /// ```
    pub fn get_coefficient_at(&self, power: u32) -> f64 {
        self.coefficients.get(&power).copied().unwrap_or(0.0)
    }

    /// Adds the specified value to the coefficient of the term with the indeterminate raised
    /// to the given power.
    ///
    /// # Examples
    ///
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let mut poly = Polynomial::from_coefficients(&vec![1.0, 3.0, -2.0]);
    /// poly.add_coefficient_at(2, 3.0);
    /// poly.add_coefficient_at(0, -1.0);
    /// assert_eq!(vec![4.0, 3.0, -3.0], poly.get_coefficients());
    /// ```
    pub fn add_coefficient_at(&mut self, power: u32, coefficient: f64) {
        self.set_coefficient_at(power, self.get_coefficient_at(power) + coefficient);
    }

    /// Subtracts the specified value from the coefficient of the term with the indeterminate
    /// raised to the given power.
    ///
    /// # Examples
    ///
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let mut poly = Polynomial::from_coefficients(&vec![1.0, 3.0, -2.0]);
    /// poly.sub_coefficient_at(2, 3.0);
    /// poly.sub_coefficient_at(0, -1.0);
    /// assert_eq!(vec![-2.0, 3.0, -1.0], poly.get_coefficients());
    /// ```
    pub fn sub_coefficient_at(&mut self, power: u32, coefficient: f64) {
        self.set_coefficient_at(power, self.get_coefficient_at(power) - coefficient);
    }

    /// Multiplies the coefficient of the term with the indeterminate raised to the given power
    /// by the specified value.
    ///
    /// # Examples
    ///
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let mut poly = Polynomial::from_coefficients(&vec![1.0, 3.0, -2.0]);
    /// poly.mul_coefficient_at(2, 3.0);
    /// poly.mul_coefficient_at(1, -2.0);
    /// poly.mul_coefficient_at(0, 0.0);
    /// assert_eq!(vec![3.0, -6.0, 0.0], poly.get_coefficients());
    /// ```
    pub fn mul_coefficient_at(&mut self, power: u32, coefficient: f64) {
        self.set_coefficient_at(power, self.get_coefficient_at(power) * coefficient);
    }

    /// Divides the coefficient of the term with the indeterminate raised to the given power
    /// by the specified value.
    ///
    /// # Examples
    ///
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let mut poly = Polynomial::from_coefficients(&vec![1.0, 3.0, -2.0]);
    /// poly.div_coefficient_at(2, 2.0);
    /// poly.div_coefficient_at(0, -2.0);
    /// assert_eq!(vec![0.5, 3.0, 1.0], poly.get_coefficients());
    /// ```
    pub fn div_coefficient_at(&mut self, power: u32, coefficient: f64) {
        self.set_coefficient_at(power, self.get_coefficient_at(power) / coefficient);
    }

    /// Creates a `Polynomial` instance from a vector of coefficients.
    ///
    /// The coefficients must specify subsequent terms sorted by their degree in descending order,
    /// with the last coefficient in the vector specifying the term of degree zero.
    ///
    /// # Examples
    ///
    /// Represent quadratic polynomial `x^2 + x -2` as a `Polynomial` instance:
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let coefficients = vec![1.0, 1.0, -2.0];
    /// let poly = Polynomial::from_coefficients(&coefficients);
    /// assert_eq!(coefficients, poly.get_coefficients());
    /// ```
    pub fn from_coefficients(coefficients: &Vec<f64>) -> Polynomial {
        let mut poly = Polynomial::zero();
        for (power, coefficient) in (0..coefficients.len()).rev().zip(coefficients.iter()) {
            poly.set_coefficient_at(power as u32, *coefficient);
        }
        poly
    }

    /// Returns a vector of coefficients.
    ///
    /// The vector starts with the first non-zero coefficient (from the highest
    /// power of the indeterminate downward), followed by all later coefficients.
    /// For the zero polynomial, this returns an empty vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let coefficients = vec![0.0, 1.0, 0.0, -2.0];
    /// let poly = Polynomial::from_coefficients(&coefficients);
    /// assert_eq!(vec![1.0, 0.0, -2.0], poly.get_coefficients());
    /// ```
    pub fn get_coefficients(&self) -> Vec<f64> {
        let mut result = Vec::new();
        let mut last_power: Option<u32> = None;
        for (power, coefficient) in self.coefficients.iter().rev() {

            // Add skipped zero coefficients
            if let Some(last_x_power) = last_power {
                let skipped_powers_count = last_x_power - *power - 1;
                if skipped_powers_count > 0 {
                    for _ in 0..skipped_powers_count {
                        result.push(0.0);
                    }
                }
            }
            result.push(*coefficient);
            last_power = Some(*power);
        }

        // Prevent the trailing zero coefficients from being skipped
        if let Some(last_x_power) = last_power {
            if last_x_power > 0 {
                for _ in 0..last_x_power {
                    result.push(0.0);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn from_coefficients_works_correctly() {
        let poly = Polynomial::from_coefficients(&vec![0.0, 2.0, 0.0, 2.0, -3.0]);
        assert_eq!(vec![2.0, 0.0, 2.0, -3.0], poly.get_coefficients());
    }

    #[test]
    fn get_coefficients_works() {
        let coefficients = vec![2.0, 0.0, 2.0, -3.0];
        let poly = Polynomial::from_coefficients(&coefficients);
        assert_eq!(coefficients, poly.get_coefficients());
    }

    #[test]
    fn get_coefficients_handles_zero_polynomial_case() {
        let poly = Polynomial::zero();
        assert_eq!(Vec::<f64>::new(), poly.get_coefficients());
    }

    #[test]
    fn get_coefficients_handles_trailing_zero_coefficients() {
        let coefficients = vec![1.0, 0.0, 0.0];
        let poly = Polynomial::from_coefficients(&coefficients);
        assert_eq!(coefficients, poly.get_coefficients());
    }
}