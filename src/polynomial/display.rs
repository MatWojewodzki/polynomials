use super::Polynomial;
use num::complex::Complex;
use num::rational::Ratio;
use num::{BigInt, One};
use num::{Integer, Num, Signed};
use std::fmt;
use std::fmt::{Display, Write};

/// Specifies the format used by the `Polynomial.format_with` method.
pub enum PolynomialFormat {
    /// Standard format that uses caret `^` before the powers.
    Standard,

    /// Latex-compatible format that uses carets `^` before the powers and curly braces `{}`
    /// around them.
    Latex,

    /// Concise format that omits the carets `^` before the powers.
    Concise,
}

pub trait CoefficientFormat {
    fn format_coefficient<T: Write>(
        &self,
        w: &mut T,
        is_leading_term: bool,
        is_last_term: bool,
        format: &PolynomialFormat,
    ) -> fmt::Result;
}

impl<T> CoefficientFormat for Ratio<T>
where
    T: Clone + Display + Integer + Signed,
{
    fn format_coefficient<K: Write>(
        &self,
        f: &mut K,
        is_leading_term: bool,
        is_last_term: bool,
        format: &PolynomialFormat,
    ) -> fmt::Result {
        // Write the sign of the term
        if is_leading_term && self.is_negative() {
            write!(f, "- ")?;
        } else if !is_leading_term {
            write!(f, " {} ", if self.is_negative() { "-" } else { "+" })?;
        }

        let numerator = self.numer().abs();
        let denominator = self.denom().abs();

        // Write the abs of the coefficient
        if !self.is_one() || is_last_term {
            match format {
                PolynomialFormat::Latex => write!(f, r"\frac{{{}}}{{{}}}", numerator, denominator)?,
                _ => write!(f, r"{}/{}", numerator, denominator)?,
            }
        }

        // Write the multiplication sign
        if !is_last_term {
            match format {
                PolynomialFormat::Latex => write!(f, r"\cdot")?,
                _ => write!(f, "*")?,
            }
        }
        Ok(())
    }
}

impl<T> CoefficientFormat for Complex<T>
where
    T: Display + Clone + Num + PartialOrd + CoefficientFormat,
{
    fn format_coefficient<K: Write>(
        &self,
        w: &mut K,
        is_leading_term: bool,
        is_last_term: bool,
        format: &PolynomialFormat,
    ) -> fmt::Result {
        if self.im.is_zero() {
            return self
                .re
                .format_coefficient(w, is_leading_term, is_last_term, format);
        }
        if self.re.is_zero() {
            self.im
                .format_coefficient(w, is_leading_term, is_last_term, format)?;
            return write!(w, "i");
        }
        if !is_leading_term {
            write!(w, " + ")?;
        }

        if !is_last_term {
            write!(w, r"({})", self)?;
        } else {
            write!(w, "{}", self)?;
        }

        if !is_last_term {
            match format {
                PolynomialFormat::Latex => write!(w, r"\cdot")?,
                _ => write!(w, "*")?,
            }
        }
        Ok(())
    }
}

macro_rules! impl_coefficient_format_for_simple_type {
    ($type:ty) => {
        impl CoefficientFormat for $type {
            fn format_coefficient<T: Write>(
                &self,
                w: &mut T,
                is_leading_term: bool,
                is_last_term: bool,
                _format: &PolynomialFormat,
            ) -> fmt::Result {
                if is_leading_term && self.is_negative() {
                    write!(w, "- ")?;
                } else if !is_leading_term {
                    write!(w, " {} ", if self.is_negative() { "-" } else { "+" })?;
                }

                if !self.abs().is_one() || is_last_term {
                    write!(w, "{}", self.abs())?;
                }
                Ok(())
            }
        }
    };
}

impl_coefficient_format_for_simple_type!(f32);
impl_coefficient_format_for_simple_type!(f64);
impl_coefficient_format_for_simple_type!(i8);
impl_coefficient_format_for_simple_type!(i16);
impl_coefficient_format_for_simple_type!(i32);
impl_coefficient_format_for_simple_type!(i64);
impl_coefficient_format_for_simple_type!(i128);
impl_coefficient_format_for_simple_type!(isize);

impl_coefficient_format_for_simple_type!(BigInt);

impl<T> Polynomial<T>
where
    T: Num + CoefficientFormat,
{
    fn write_to_fmt<K: Write>(&self, w: &mut K, string_format: PolynomialFormat) -> fmt::Result {
        // Handle the zero polynomial case
        if let None = self.degree() {
            return write!(w, "0");
        }

        for (power, coefficient) in self.coefficients.iter().rev() {
            if coefficient.is_zero() {
                continue;
            }

            coefficient.format_coefficient(
                w,
                *power == self.degree().unwrap(),
                *power == 0,
                &string_format,
            )?;

            // Write the indeterminate x and the power if it's not 0
            if *power == 0 {
                continue;
            }
            if *power == 1 {
                write!(w, "x")?;
                continue;
            }
            match string_format {
                PolynomialFormat::Latex => write!(w, "x^{{{power}}}")?,
                PolynomialFormat::Concise => write!(w, "x{power}")?,
                PolynomialFormat::Standard => write!(w, "x^{power}")?,
            }
        }
        Ok(())
    }

    /// Returns the polynomial as a [`String`] in the specified [format](PolynomialFormat).
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

impl<T> Display for Polynomial<T>
where
    T: Num + CoefficientFormat,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to_fmt(f, PolynomialFormat::Standard)
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;
    use crate::PolynomialFormat;
    use num::Complex;
    use num::rational::Ratio;

    #[test]
    fn to_string_float() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 2.5, -3.0]);
        assert_eq!("x^2 + 2.5x - 3", poly.to_string());
    }

    #[test]
    fn to_string_rational() {
        let poly = Polynomial::from_coefficients(&vec![
            Ratio::new_raw(-1, 2),
            Ratio::new_raw(3, 7),
            Ratio::new_raw(-1, 3),
        ]);
        assert_eq!("- 1/2*x^2 + 3/7*x - 1/3", poly.to_string());
    }

    #[test]
    fn to_string_complex() {
        let poly = Polynomial::from_coefficients(&vec![
            Complex::new(1.0, -2.5),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, -2.0),
            Complex::new(-3.0, -2.0),
            Complex::new(2.0, -6.0),
        ]);
        assert_eq!(
            "(1-2.5i)*x^4 + x^3 - 2ix^2 + (-3-2i)*x + 2-6i",
            poly.to_string()
        );
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
        let poly: Polynomial<f64> = Polynomial::zero();
        assert_eq!("0", poly.to_string());
    }

    #[test]
    fn format_with_latex_float() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        assert_eq!("x^{2} + 2x - 3", poly.format_with(PolynomialFormat::Latex));
    }

    #[test]
    fn format_with_concise_works() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        assert_eq!("x2 + 2x - 3", poly.format_with(PolynomialFormat::Concise));
    fn format_with_latex_rational() {
        let poly = Polynomial::from_coefficients(&vec![
            Ratio::new_raw(-1, 2),
            Ratio::new_raw(3, 7),
            Ratio::new_raw(-4, 3),
        ]);
        assert_eq!(
            r"- \frac{1}{2}\cdotx^{2} + \frac{3}{7}\cdotx - \frac{4}{3}",
            poly.format_with(PolynomialFormat::Latex)
        )
    }

    #[test]
    fn format_with_concise_float() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 2.5, -3.0]);
        assert_eq!("x2 + 2.5x - 3", poly.format_with(PolynomialFormat::Concise));
    }

    #[test]
    fn format_with_concise_rational() {
        let poly = Polynomial::from_coefficients(&vec![
            Ratio::new_raw(-1, 2),
            Ratio::new_raw(3, 7),
            Ratio::new_raw(-4, 3),
        ]);
        assert_eq!(
            "- 1/2*x2 + 3/7*x - 4/3",
            poly.format_with(PolynomialFormat::Concise)
        )
    }
    }
}
