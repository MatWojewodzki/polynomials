use super::Polynomial;
use num::Num;
use regex::Regex;
use std::fmt::Debug;
use std::str::FromStr;

impl<T> FromStr for Polynomial<T>
where
    T: Clone + FromStr + Num,
    <T as FromStr>::Err: Debug,
{
    type Err = &'static str;

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
        let mut poly = Polynomial::zero();
        let err = Err("Invalid string format.");

        let pat = r"(?<sign>[+-])[ \n]*(?<coefficient>.+)?[ \n]*\*?[ \n]*(?<variable>x)?(?:\^?(?<power>\d+))?";
        let re = Regex::new(pat).unwrap();

        // Add a trailing sign if it is not present
        let string = if let Some(c) = string.trim().chars().next() {
            if c == '-' || c == '+' {
                string.trim()
            } else {
                &format!("+ {}", string.trim())
            }
        } else {
            return Ok(poly);
        };

        let mut captured_terms = String::new();

        for caps in re.captures_iter(string) {
            captured_terms.push_str(&caps[0]);

            let sign: T = match caps.name("sign").unwrap().as_str() {
                "+" => T::one(),
                "-" => T::zero() - T::one(),
                _ => panic!("Sign was supposed to be '+' or '-'."),
            };

            let coefficient: Option<T> = if let Some(mat) = caps.name("coefficient") {
                // TODO: Proper error handling needed here instead of unwrap
                Some(T::from_str(mat.as_str()).unwrap())
            } else {
                None
            };

            let variable: Option<char> = if let Some(mat) = caps.name("variable") {
                Some(mat.as_str().chars().next().unwrap())
            } else if coefficient.is_none() {
                return err;
            } else {
                None
            };

            let power: u32 = if let Some(mat) = caps.name("power") {
                mat.as_str().parse().unwrap()
            } else if variable.is_none() {
                0
            } else {
                1
            };

            // In the case of no coefficient default to 1
            let coefficient = if let Some(coefficient) = coefficient {
                coefficient
            } else {
                T::one()
            };

            poly.add_coefficient_at(power, coefficient * sign);
        }

        // Compare captured terms and input string with spaces and newlines removed
        let captured_terms = captured_terms.replace(" ", "").replace("\n", "");
        let string = string.replace(" ", "").replace("\n", "");

        if captured_terms != string {
            return err;
        }

        Ok(poly)
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;
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
    fn from_string_with_repeated_terms() {
        let poly = Polynomial::from_str("x^2 + x + x^2 - x + 5 - 10").unwrap();
        assert_eq!(vec![2.0, 0.0, -5.0], poly.get_coefficients());
    }

    #[test]
    fn from_string_invalid_formats() {
        assert!(Polynomial::<f64>::from_str("x^2 + + 3x").is_err());
        assert!(Polynomial::<f64>::from_str("2y^2 + 3y").is_err());
        assert!(Polynomial::<f64>::from_str("2x^2.5").is_err());
    }

    #[test]
    fn from_string_empty() {
        let poly: Polynomial<f64> = Polynomial::from_str("").unwrap();
        assert!(poly.is_zero());
    }
}
