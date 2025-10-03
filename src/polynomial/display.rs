use std::fmt;
use std::fmt::Display;
use super::Polynomial;

/// Specifies the format used by the `Polynomial.format_with` method.
pub enum PolynomialFormat {
    /// Standard format that uses caret `^` before the powers.
    Standard,

    /// Latex-compatible format that uses carets `^` before the powers and curly braces `{}`
    /// around them.
    Latex,

    /// Concise format that omits the carets `^` before the powers.
    Concise
}

impl Polynomial {
    fn write_to_fmt(&self, f: &mut dyn fmt::Write, string_format: PolynomialFormat) -> fmt::Result {
        // Handle the zero polynomial case
        if let None = self.degree() {
            return write!(f, "0");
        }

        for (power, coefficient) in self.coefficients.iter().rev() {
            if *coefficient == 0.0 {
                continue;
            }

            let sign = if *coefficient > 0.0 { "+" } else { "-" };

            // Write the sign of the term
            if *power == self.degree().unwrap() && sign == "-" {
                write!(f, "{sign} ")?;
            } else if *power != self.degree().unwrap() {
                write!(f, " {sign} ")?;
            }

            // Write the coefficient if it's not 1, or it's the term of degree 0
            if coefficient.abs() != 1.0 || *power == 0 {
                write!(f, "{}", coefficient.abs())?;
            }

            // Write the indeterminate x and the power if it's not 0
            if *power == 0 {
                continue;
            }
            if *power == 1 {
                write!(f, "x")?;
                continue;
            }
            match string_format {
                PolynomialFormat::Latex => write!(f, "x^{{{power}}}")?,
                PolynomialFormat::Concise => write!(f, "x{power}")?,
                PolynomialFormat::Standard => write!(f, "x^{power}")?,
            }
        }
        Ok(())
    }

    /// Returns the polynomial as a `String` in the specified format.
    ///
    /// # Examples
    ///
    /// Format the polynomial using the `PolynomialFormat::Standard` format:
    /// ```
    /// use polynomials::{Polynomial, PolynomialFormat};
    ///
    /// let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -1.0, 3.0]);
    /// assert_eq!("x^3 + 2x^2 - x + 3", poly.format_with(PolynomialFormat::Standard));
    /// ```
    /// 
    /// Format the polynomial using the `PolynomialFormat::Latex` format:
    /// ```
    /// use polynomials::{Polynomial, PolynomialFormat};
    /// 
    /// let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -1.0, 3.0]);
    /// assert_eq!("x^{3} + 2x^{2} - x + 3", poly.format_with(PolynomialFormat::Latex));
    /// ```
    /// 
    /// Format the polynomial using the `PolynomialFormat::Concise` format:
    /// ```
    /// use polynomials::{Polynomial, PolynomialFormat};
    /// 
    /// let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -1.0, 3.0]);
    /// assert_eq!("x3 + 2x2 - x + 3", poly.format_with(PolynomialFormat::Concise));
    /// ```
    pub fn format_with(&self, format: PolynomialFormat) -> String {
        let mut buffer = String::new();
        let _ = self.write_to_fmt(&mut buffer, format);
        buffer
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to_fmt(f, PolynomialFormat::Standard)
    }
}

#[cfg(test)]
mod tests {
    use crate::PolynomialFormat;
    use super::Polynomial;

    #[test]
    fn to_string_handles_general_case() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        assert_eq!("x^2 + 2x - 3", poly.to_string());
    }

    #[test]
    fn to_string_handles_single_coefficient() {
        let poly = Polynomial::from_coefficients(&vec![5.0]);
        assert_eq!("5", poly.to_string());
    }

    #[test]
    fn to_string_handles_negative_coefficients() {
        let poly = Polynomial::from_coefficients(&vec![-2.0, -3.0, -1.0]);
        assert_eq!("- 2x^2 - 3x - 1", poly.to_string());
    }

    #[test]
    fn to_string_handles_coefficient_one() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 0.0, 0.0]);
        assert_eq!("x^2", poly.to_string());

        let poly = Polynomial::from_coefficients(&vec![-1.0]);
        assert_eq!("- 1", poly.to_string());
    }

    #[test]
    fn to_string_handles_high_degree() {
        let mut poly = Polynomial::zero();
        poly.set_coefficient_at(10, 2.0);
        poly.set_coefficient_at(5, -3.0);
        poly.set_coefficient_at(0, 1.0);
        assert_eq!("2x^10 - 3x^5 + 1", poly.to_string());
    }

    #[test]
    fn to_string_handles_zero_polynomial() {
        let poly = Polynomial::zero();
        assert_eq!("0", poly.to_string());
    }

    #[test]
    fn format_with_latex_works() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        assert_eq!("x^{2} + 2x - 3", poly.format_with(PolynomialFormat::Latex));
    }

    #[test]
    fn format_with_concise_works() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        assert_eq!("x2 + 2x - 3", poly.format_with(PolynomialFormat::Concise));
    }
}