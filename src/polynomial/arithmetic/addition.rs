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

#[cfg(test)]
mod tests {
    use super::Polynomial;
    
    #[test]
    fn polynomial_add() {
        let poly1 = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly2 = Polynomial::from_coefficients(&vec![-2.0, -2.0, -1.0]);
        let poly3 = poly1 + poly2;
        assert!(poly3 == Polynomial::from_coefficients(&vec![-1.0, 0.0, -4.0]));
    }

    #[test]
    fn polynomial_add_assign() {
        let mut poly1 = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly2 = Polynomial::from_coefficients(&vec![-2.0, -2.0, -1.0]);
        poly1 += poly2;
        assert!(poly1 == Polynomial::from_coefficients(&vec![-1.0, 0.0, -4.0]));
    }
}