use super::Polynomial;
use num::Num;
use regex::Regex;
use std::fmt::Debug;
use std::ops::Neg;
use std::str::FromStr;

#[derive(Debug)]
pub struct PolynomialParsingError(String);

fn parse_coefficient<T: FromStr>(coefficient_str: &str) -> Result<T, PolynomialParsingError> {
    Ok(T::from_str(coefficient_str).map_err(|_| {
        PolynomialParsingError(format!(
            "could not parse '{}' as a coefficient",
            coefficient_str
        ))
    })?)
}

impl<T> FromStr for Polynomial<T>
where
    T: Clone + Neg<Output = T> + FromStr + Num,
    <T as FromStr>::Err: Debug,
{
    type Err = PolynomialParsingError;

    /// Constructs a new instance from a given polynomial string representation.
    ///
    /// The function returns `Ok(Polynomial)` if parsing is successful or `Err(&str)` if the string
    /// format was incorrect.
    ///
    /// # Supported string format
    ///
    /// The string must follow the pattern `<term> +/- <term> +/- ... +/- <term>` where each `<term>`
    /// is of the form `<coefficient>x<power>`. Spaces between the terms and plus or minus signs
    /// are optional. Terms of degree equal to one may be written as `<coefficient>x` or
    /// `<coefficient>x1`, while in the terms of degree zero the `x` might be omitted entirely.
    ///
    /// Additionally:
    /// - An asterisk `*` sign might be inserted after the coefficient, with optional
    /// spaces around.
    /// - A caret `^` character may be inserted before the power.
    ///
    /// Terms of the same degree may occur multiple times in the string.
    /// Only the character `x` may be used as an indeterminate.
    ///
    /// # Examples
    ///
    /// Create a polynomial from a string with spaces between the terms:
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let poly = Polynomial::from_str("-3x2 + 4x - 5").unwrap();
    /// assert_eq!(vec![-3.0, 4.0, -5.0], poly.get_coefficients());
    /// ```
    ///
    /// Parse a string which uses concise syntax without spaces:
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let poly = Polynomial::from_str("2x5-x4+4x2-3").unwrap();
    /// assert_eq!(vec![2.0, -1.0, 0.0, 4.0, 0.0, -3.0], poly.get_coefficients());
    /// ```
    ///
    /// Parse a string which uses carets before the powers:
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let poly = Polynomial::from_str("x^4 - x^2 + x").unwrap();
    /// assert_eq!(vec![1.0, 0.0, -1.0, 1.0, 0.0], poly.get_coefficients());
    /// ```
    ///
    /// Parse a string which uses asterisks:
    /// ```
    /// use polynomials::Polynomial;
    ///
    /// let poly = Polynomial::from_str("-2 * x^2 -3*x + 5").unwrap();
    /// ```
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string = string
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        let mut poly = Polynomial::zero();

        if string.is_empty() {
            return Ok(poly);
        }

        let term_pat = r"((?<bracket_sign>[+-])?(?<opening_bracket>\())?(?<coefficient>[0-9.ij+\-/]+)?(?<closing_bracket>\))?\*?(?<indeterminate>x)?(\^?\{?(?<exponent>\d+)}?)?";
        let indeterminate_re = Regex::new(term_pat).unwrap();

        for caps in indeterminate_re.captures_iter(&string) {
            if caps.name("opening_bracket").is_some() != caps.name("closing_bracket").is_some() {
                return Err(PolynomialParsingError(format!(
                    "the bracket was not closed in the term '{}'",
                    &caps[0]
                )));
            }

            let bracket_positive = if let Some(mat) = caps.name("bracket_sign") {
                match mat.as_str() {
                    "+" => true,
                    "-" => false,
                    _ => panic!("Sign was supposed to be '+' or '-'."),
                }
            } else {
                true
            };

            let coefficient: T = if let Some(mat) = caps.name("coefficient") {
                match mat.as_str() {
                    "+" => T::one(),
                    "-" => -T::one(),
                    _ => parse_coefficient(mat.as_str())?,
                }
            } else {
                T::one()
            };

            let coefficient = if bracket_positive {
                coefficient
            } else {
                -coefficient
            };

            let indeterminate: bool = caps.name("indeterminate").is_some();

            let exponent: u32 = if let Some(mat) = caps.name("exponent") {
                mat.as_str().parse().unwrap() // Exponent should always be an integer
            } else if !indeterminate {
                0
            } else {
                1
            };

            poly.add_coefficient_at(exponent, coefficient);
        }
        Ok(poly)
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;
    use num::Complex;
    use num::rational::Ratio;
    use std::str::FromStr;

    #[test]
    fn from_string_integer_coefficients() {
        let poly = Polynomial::from_str("-x^4 - 2x^3 + 10x2 - x + 5").unwrap();
        assert_eq!(vec![-1.0, -2.0, 10.0, -1.0, 5.0], poly.get_coefficients());
    }

    #[test]
    fn from_string_decimal_coefficients() {
        let poly = Polynomial::from_str("1.5x^2 - 0.5x + 2.125").unwrap();
        assert_eq!(vec![1.5, -0.5, 2.125], poly.get_coefficients());
    }

    #[test]
    fn from_string_concise_spacing() {
        let poly = Polynomial::from_str("x^2+x-5").unwrap();
        assert_eq!(vec![1.0, 1.0, -5.0], poly.get_coefficients());
    }

    #[test]
    fn from_string_omitted_carets() {
        let poly = Polynomial::from_str("x4 - 2x3 + 5x2 - x").unwrap();
        assert_eq!(vec![1.0, -2.0, 5.0, -1.0, 0.0], poly.get_coefficients());
    }

    #[test]
    fn from_string_with_asterisks() {
        let poly = Polynomial::from_str("- 2 * x^2 -3*x + 5").unwrap();
        assert_eq!(vec![-2.0, -3.0, 5.0], poly.get_coefficients());
    }

    #[test]
    fn from_string_empty() {
        let poly: Polynomial<f64> = Polynomial::from_str("").unwrap();
        assert!(poly.is_zero());
    }

    #[test]
    fn from_string_rational() {
        let poly: Polynomial<Ratio<i32>> =
            Polynomial::from_str("-1/2x3 + 3/-4x2 -x - 7/5").unwrap();
        assert_eq!(
            vec![
                Ratio::new(-1, 2),
                Ratio::new(3, -4),
                Ratio::new(-1, 1),
                Ratio::new(-7, 5)
            ],
            poly.get_coefficients()
        )
    }

    #[test]
    fn from_string_complex() {
        let poly: Polynomial<Complex<i32>> =
            Polynomial::from_str("(-1 + 2i)x^4 - (3 - i)x3 + 5x2 - ix - 5 + 2i").unwrap();
        assert_eq!(
            vec![
                Complex::new(-1, 2),
                Complex::new(-3, 1),
                Complex::new(5, 0),
                Complex::new(0, -1),
                Complex::new(-5, 2)
            ],
            poly.get_coefficients()
        )
    }

    #[test]
    fn from_string_invalid_formats() {
        let err = Polynomial::<f64>::from_str("1.5.2x").unwrap_err();
        assert!(err.0.contains("'1.5.2'"));

        let err = Polynomial::<Complex<f64>>::from_str("+(1-ix").unwrap_err();
        assert!(err.0.contains("not closed"));
    }
}
