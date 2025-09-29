use std::ops::{Add, AddAssign};
use super::super::Polynomial;

fn add_in_place(poly1: &mut Polynomial, poly2: &Polynomial) {
    for (power, coefficient) in poly2.coefficients.iter() {
        poly1.add_coefficient_at(*power, *coefficient);
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(mut self, other: Self) -> Self::Output {
        add_in_place(&mut self, &other);
        self
    }
}

impl Add<f64> for Polynomial {
    type Output = Polynomial;
    
    fn add(mut self, other: f64) -> Self::Output {
        self.add_coefficient_at(0, other);
        self
    }
}

impl Add<i32> for Polynomial {
    type Output = Polynomial;
    
    fn add(mut self, other: i32) -> Self::Output {
        self.add_coefficient_at(0, other as f64);
        self
    }
}

impl AddAssign for Polynomial {
    fn add_assign(&mut self, other: Self) {
        add_in_place(self, &other);    
    }
}

impl AddAssign<f64> for Polynomial {
    fn add_assign(&mut self, other: f64) {
        self.add_coefficient_at(0, other);
    }   
}

impl AddAssign<i32> for Polynomial {
    fn add_assign(&mut self, other: i32) {
        self.add_coefficient_at(0, other as f64);
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn add() {
        let poly1 = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly2 = Polynomial::from_coefficients(&vec![-2.0, -2.0, -1.0]);
        let poly3 = poly1 + poly2;
        assert_eq!(vec![-1.0, 0.0, -4.0], poly3.get_coefficients());
    }

    #[test]
    fn add_float() {
        let poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        let poly_plus_two = poly + 2.0;
        assert_eq!(vec![-2.0, 0.0, 3.0], poly_plus_two.get_coefficients());
    }

    #[test]
    fn add_int() {
        let poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        let poly_plus_two = poly + 2;
        assert_eq!(vec![-2.0, 0.0, 3.0], poly_plus_two.get_coefficients());
    }

    #[test]
    fn add_assign() {
        let mut poly1 = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly2 = Polynomial::from_coefficients(&vec![-2.0, -2.0, -1.0]);
        poly1 += poly2;
        assert_eq!(vec![-1.0, 0.0, -4.0], poly1.get_coefficients());
    }

    #[test]
    fn add_assign_float() {
        let mut poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        poly += 2.0;
        assert_eq!(vec![-2.0, 0.0, 3.0], poly.get_coefficients());
    }

    #[test]
    fn add_assign_int() {
        let mut poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        poly += 2;
        assert_eq!(vec![-2.0, 0.0, 3.0], poly.get_coefficients());
    }
}