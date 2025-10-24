use num::{BigInt, BigUint, One, Zero};
use std::fmt;
use std::fmt::{Display, Write};
use num::complex::Complex;
use num::{Integer, Num, Signed};
use num::rational::Ratio;
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

pub trait CoefficientFormat {
    fn format_coefficient<T: Write>(
        &self,
        w: &mut T,
        is_leading_term: bool,
        is_last_term: bool,
        format: &PolynomialFormat
    ) -> fmt::Result;
}

trait UniversalAbs {
    fn abs_universal(&self) -> Self;
}

macro_rules! impl_is_negative_signed {
    ($type:ty) => {
        impl UniversalAbs for $type {
            fn abs_universal(&self) -> Self {
                self.abs()
            }
        }
    };
}

macro_rules! impl_is_negative_unsigned {
    ($type:ty) => {
        impl UniversalAbs for $type {
            fn abs_universal(&self) -> Self {
                *self
            }
        }
    };
}

impl_is_negative_signed!(i8);
impl_is_negative_signed!(i16);
impl_is_negative_signed!(i32);
impl_is_negative_signed!(i64);
impl_is_negative_signed!(i128);
impl_is_negative_signed!(isize);

impl_is_negative_unsigned!(u8);
impl_is_negative_unsigned!(u16);
impl_is_negative_unsigned!(u32);
impl_is_negative_unsigned!(u64);
impl_is_negative_unsigned!(u128);
impl_is_negative_unsigned!(usize);

impl<T> CoefficientFormat for Ratio<T>
where
    T: Clone + Display + Integer + UniversalAbs
{
    fn format_coefficient<K: Write>(
        &self,
        f: &mut K,
        is_leading_term: bool,
        is_last_term: bool,
        format: &PolynomialFormat
    ) -> fmt::Result {

        let is_negative = self < &Ratio::zero();

        if is_leading_term && is_negative {
            write!(f, "- ")?;
        } else if !is_leading_term {
            write!(f, "{} ", if is_negative { "-" } else { "+" })?;
        }

        let numerator = self.numer().abs_universal();
        let denominator = self.denom().abs_universal();

        if !self.is_one() || is_last_term {
            match format {
                PolynomialFormat::Latex => write!(f, r"\frac{}{}\cdot", numerator, denominator)?,
                _ => write!(f, r"{}/{}\cdot", numerator, denominator)?
            }
        }
        Ok(())
    }
}

impl<T> CoefficientFormat for Complex<T>
where
    T: Display + Clone + Num + PartialOrd
{
    fn format_coefficient<K: Write>(
        &self,
        w: &mut K,
        is_leading_term: bool,
        _is_last_term: bool,
        _format: &PolynomialFormat
    ) -> fmt::Result {
        if !is_leading_term {
            write!(w, "+ ")?;
        }

        write!(w, "({})*", self)?;
        Ok(())
    }
}

macro_rules! impl_coefficient_format_signed {
    ($type:ty) => {
        impl CoefficientFormat for $type {
            fn format_coefficient<T: Write>(
                &self,
                w: &mut T,
                is_leading_term: bool,
                is_last_term: bool,
                _format: &PolynomialFormat
            ) -> fmt::Result {
                if is_leading_term && self.is_negative() {
                    write!(w, "- ")?;
                } else if !is_leading_term {
                    write!(w, "{} ", if self.is_negative() { "-" } else { "+" })?;
                }

                if !self.abs().is_one() || is_last_term {
                    write!(w, "{}", self.abs())?;
                }
                Ok(())
            }
        }
    };
}

macro_rules! impl_coefficient_format_unsigned {
    ($type:ty) => {
        impl CoefficientFormat for $type {
            fn format_coefficient<T: Write>(
                &self,
                w: &mut T,
                is_leading_term: bool,
                is_last_term: bool,
                _format: &PolynomialFormat
            ) -> fmt::Result {
                if !is_leading_term {
                    write!(w, "+ ")?;
                }

                if !self.is_one() || is_last_term {
                    write!(w, "{}", self)?;
                }
                Ok(())
            }
        }
    };
}

impl_coefficient_format_signed!(f32);
impl_coefficient_format_signed!(f64);
impl_coefficient_format_signed!(i8);
impl_coefficient_format_signed!(i16);
impl_coefficient_format_signed!(i32);
impl_coefficient_format_signed!(i64);
impl_coefficient_format_signed!(i128);
impl_coefficient_format_signed!(isize);

impl_coefficient_format_unsigned!(u8);
impl_coefficient_format_unsigned!(u16);
impl_coefficient_format_unsigned!(u32);
impl_coefficient_format_unsigned!(u64);
impl_coefficient_format_unsigned!(u128);
impl_coefficient_format_unsigned!(usize);

impl_coefficient_format_signed!(BigInt);
impl_coefficient_format_unsigned!(BigUint);

impl<T> Polynomial<T>
where
    T: Num + CoefficientFormat
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
                &string_format
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
    T: Num + CoefficientFormat
{
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
        let poly: Polynomial<f64> = Polynomial::zero();
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