use std::collections::HashMap;
use std::fmt::{self, Display};
use regex::Regex;

pub struct Polynomial {
    coefficients: HashMap<u32, f64>,
}

impl Polynomial {
    pub fn zero() -> Polynomial {
        Polynomial {
            coefficients: HashMap::new(),
        }
    }

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

    pub fn from_coefficients(coefficients: &Vec<f64>) -> Polynomial {
        let mut poly = Polynomial::zero();
        for (power, coefficient) in (0..coefficients.len()).rev().zip(coefficients.iter()) {
            poly.set_coefficient_at(power as u32, *coefficient);
        }
        poly
    }

    pub fn is_zero(&self) -> bool {
        self.coefficients.is_empty()
    }

    pub fn degree(&self) -> Option<u32> {
        self.coefficients.keys().max().copied()
    }
    
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

impl Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let None = self.degree() {
            return write!(f, "0");
        }
        
        let mut powers: Vec<_> = self.coefficients.keys().collect();
        powers.sort_by(|a, b| b.cmp(a));

        for power in powers {
            let coefficient = self.coefficients.get(power).unwrap();

            if *coefficient == 0.0 {
                continue;
            }

            let sign = if *coefficient > 0.0 { "+" } else { "-" };

            if *power == self.degree().unwrap() && sign == "-" {
                write!(f, "{sign} ")?;
            } else if *power != self.degree().unwrap() {
                write!(f, " {sign} ")?;
            }

            if coefficient.abs() != 1.0 || *power == 0 {
                write!(f, "{}", coefficient.abs())?;
            }

            if *power > 1 {
                write!(f, "x^{power}")?;
            } else if *power == 1 {
                write!(f, "x")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn to_string_handles_general_case() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        assert_eq!(poly.to_string(), String::from("x^2 + 2x - 3"));
    }

    #[test]
    fn to_string_handles_single_coefficient() {
        let poly = Polynomial::from_coefficients(&vec![5.0]);
        assert_eq!(poly.to_string(), String::from("5"));
    }

    #[test]
    fn to_string_handles_negative_coefficients() {
        let poly = Polynomial::from_coefficients(&vec![-2.0, -3.0, -1.0]);
        assert_eq!(poly.to_string(), String::from("- 2x^2 - 3x - 1"));
    }

    #[test]
    fn to_string_handles_coefficient_one() {
        let mut poly = Polynomial::zero();
        poly.set_coefficient_at(2, 1.0);
        assert_eq!(poly.to_string(), String::from("x^2"));
        
        let poly = Polynomial::from_coefficients(&vec![-1.0]);
        assert_eq!(poly.to_string(), String::from("- 1"));
    }

    #[test]
    fn to_string_handles_high_degree() {
        let mut poly = Polynomial::zero();
        poly.set_coefficient_at(10, 2.0);
        poly.set_coefficient_at(5, -3.0);
        poly.set_coefficient_at(0, 1.0);
        assert_eq!(poly.to_string(), String::from("2x^10 - 3x^5 + 1"));
    }

    #[test]
    fn to_string_handles_zero_polynomial() {
        let poly = Polynomial::zero();
        assert_eq!(poly.to_string(), "0");
    }
    
    #[test]
    fn from_string_integer_coefficients() {
        let poly = Polynomial::from_string("-x^4 - 2x^3 + 10x2 - x + 5").unwrap();
        assert_eq!(poly.to_string(), String::from("- x^4 - 2x^3 + 10x^2 - x + 5"));
    }

    #[test]
    fn from_string_decimal_coefficients() {
        let poly = Polynomial::from_string("1.5x^2 - 0.5x + 2.125").unwrap();
        assert_eq!(poly.to_string(), String::from("1.5x^2 - 0.5x + 2.125"));
    }

    #[test]
    fn from_string_concise_spacing() {
        let poly = Polynomial::from_string("x^2+x-5").unwrap();
        assert_eq!(poly.to_string(), String::from("x^2 + x - 5"));
    }

    #[test]
    fn from_string_omitted_carets() {
        let poly = Polynomial::from_string("x4 - 2x3 + 5x2 - x").unwrap();
        assert_eq!(poly.to_string(), String::from("x^4 - 2x^3 + 5x^2 - x"));
    }

    #[test]
    fn from_string_with_asterisks() {
        let poly = Polynomial::from_string("- 2 * x^2 -3*x + 5").unwrap();
        assert_eq!(poly.to_string(), String::from("- 2x^2 - 3x + 5"));
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
