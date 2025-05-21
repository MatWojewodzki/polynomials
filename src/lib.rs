use std::collections::HashMap;
use std::fmt::{self, Display};

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
}
