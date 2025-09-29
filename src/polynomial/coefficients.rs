use super::Polynomial;

impl Polynomial {
    pub fn set_coefficient_at(&mut self, power: u32, coefficient: f64) {
        if coefficient == 0.0 {
            self.coefficients.remove(&power);
            return;
        }
        self.coefficients.insert(power, coefficient);
    }

    pub fn get_coefficient_at(&self, power: u32) -> f64 {
        self.coefficients.get(&power).copied().unwrap_or(0.0)
    }

    pub fn add_coefficient_at(&mut self, power: u32, coefficient: f64) {
        self.set_coefficient_at(power, self.get_coefficient_at(power) + coefficient);
    }

    pub fn sub_coefficient_at(&mut self, power: u32, coefficient: f64) {
        self.set_coefficient_at(power, self.get_coefficient_at(power) - coefficient);
    }

    pub fn mul_coefficient_at(&mut self, power: u32, coefficient: f64) {
        self.set_coefficient_at(power, self.get_coefficient_at(power) * coefficient);
    }

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