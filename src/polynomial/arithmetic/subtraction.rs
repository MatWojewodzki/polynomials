use std::ops::{Sub, SubAssign};
use super::Polynomial;

fn subtract_in_place(poly1: &mut Polynomial, poly2: &Polynomial) {
    for (power, coefficient) in poly2.coefficients.iter() {
        poly1.sub_coefficient_at(*power, *coefficient);
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;

    fn sub(mut self, other: Self) -> Self::Output {
        subtract_in_place(&mut self, &other);
        self
    }
}

impl Sub<f64> for Polynomial {
    type Output = Polynomial;

    fn sub(mut self, other: f64) -> Self::Output {
        self.sub_coefficient_at(0, other);
        self
    }
}

impl Sub<i32> for Polynomial {
    type Output = Polynomial;

    fn sub(mut self, other: i32) -> Self::Output {
        self.sub_coefficient_at(0, other as f64);
        self
    }
}

impl SubAssign for Polynomial {
    fn sub_assign(&mut self, other: Self) {
        subtract_in_place(self, &other);
    }
}

impl SubAssign<f64> for Polynomial {
    fn sub_assign(&mut self, other: f64) {
        self.sub_coefficient_at(0, other);
    }
}

impl SubAssign<i32> for Polynomial {
    fn sub_assign(&mut self, other: i32) {
        self.sub_coefficient_at(0, other as f64);
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn sub() {
        let poly1 = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly2 = Polynomial::from_coefficients(&vec![-2.0, 2.0, -1.0]);
        let poly3 = poly1 - poly2;
        assert_eq!(vec![3.0, 0.0, -2.0], poly3.get_coefficients());
    }

    #[test]
    fn sub_float() {
        let poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        let poly_minus_two = poly - 2.0;
        assert_eq!(vec![-2.0, 0.0, -1.0], poly_minus_two.get_coefficients());
    }

    #[test]
    fn sub_int() {
        let poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        let poly_minus_two = poly - 2;
        assert_eq!(vec![-2.0, 0.0, -1.0], poly_minus_two.get_coefficients());
    }

    #[test]
    fn sub_assign() {
        let mut poly1 = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly2 = Polynomial::from_coefficients(&vec![-2.0, 2.0, -1.0]);
        poly1 -= poly2;
        assert_eq!(vec![3.0, 0.0, -2.0], poly1.get_coefficients());
    }

    #[test]
    fn sub_assign_float() {
        let mut poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        poly -= 2.0;
        assert_eq!(vec![-2.0, 0.0, -1.0], poly.get_coefficients());
    }

    #[test]
    fn sub_assign_int() {
        let mut poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        poly -= 2;
        assert_eq!(vec![-2.0, 0.0, -1.0], poly.get_coefficients());
    }
}