use regex::Regex;
use super::Polynomial;

impl Polynomial {
    pub fn from_string(string: &str) -> Result<Polynomial, &str> {

        let mut poly = Polynomial::zero();
        let err = Err("Invalid string format.");

        let pat = r"(?<sign>[+-])[ \n]*(?<coefficient>\d+(\.\d*)?)?[ \n]*\*?[ \n]*(?<variable>x)?(?:\^?(?<power>\d+))?";
        let re = Regex::new(pat).unwrap();

        // Add a trailing sign if it is not present
        let string = if let Some(c) = string.trim().chars().next() {
            if c == '-' || c == '+' {
                string.trim()
            } else {
                &format!("+ {}", string)
            }
        } else{
            return Ok(poly)
        };

        let mut captured_terms = String::new();

        for caps in re.captures_iter(string) {

            captured_terms.push_str(&caps[0]);

            let sign: i8 = match caps.name("sign").unwrap().as_str() {
                "+" => 1,
                "-" => -1,
                _ => panic!("Sign was supposed to be '+' or '-'.")
            };

            let coefficient: Option<f64> = if let Some(mat) = caps.name("coefficient") {
                Some(mat.as_str().parse().unwrap())
            } else {
                None
            };

            let variable: Option<char> = if let Some(mat) = caps.name("variable") {
                Some(mat.as_str().chars().next().unwrap())
            } else if coefficient.is_none() {
                return err
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

            // In the case of no coefficient default to 1.0
            let coefficient = if let Some(coefficient) = coefficient {
                coefficient
            } else {
                1.0
            };

            poly.set_coefficient_at(power, coefficient * sign as f64);
        }

        // Compare captured terms and input string with spaces and newlines removed
        let captured_terms = captured_terms.replace(" ", "").replace("\n", "");
        let string = string.replace(" ", "").replace("\n", "");

        if captured_terms != string {
            return err
        }

        Ok(poly)
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn from_string_integer_coefficients() {
        let poly = Polynomial::from_string("-x^4 - 2x^3 + 10x2 - x + 5").unwrap();
        assert_eq!(vec![-1.0, -2.0, 10.0, -1.0, 5.0], poly.get_coefficients());
    }

    #[test]
    fn from_string_decimal_coefficients() {
        let poly = Polynomial::from_string("1.5x^2 - 0.5x + 2.125").unwrap();
        assert_eq!(vec![1.5, -0.5, 2.125], poly.get_coefficients());
    }

    #[test]
    fn from_string_concise_spacing() {
        let poly = Polynomial::from_string("x^2+x-5").unwrap();
        assert_eq!(vec![1.0, 1.0, -5.0], poly.get_coefficients());
    }

    #[test]
    fn from_string_omitted_carets() {
        let poly = Polynomial::from_string("x4 - 2x3 + 5x2 - x").unwrap();
        assert_eq!(vec![1.0, -2.0, 5.0, -1.0, 0.0], poly.get_coefficients());
    }

    #[test]
    fn from_string_with_asterisks() {
        let poly = Polynomial::from_string("- 2 * x^2 -3*x + 5").unwrap();
        assert_eq!(vec![-2.0, -3.0, 5.0], poly.get_coefficients());
    }

    #[test]
    fn from_string_invalid_formats() {
        assert!(Polynomial::from_string("x^2 + + 3x").is_err());
        assert!(Polynomial::from_string("2y^2 + 3y").is_err());
        assert!(Polynomial::from_string("2x^2.5").is_err());
    }

    #[test]
    fn from_string_empty() {
        let poly = Polynomial::from_string("").unwrap();
        assert!(poly.is_zero());
    }
}