use std::fmt;
use std::fmt::Display;
use super::Polynomial;

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
    use super::Polynomial;

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