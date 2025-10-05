use std::ops::{Div, DivAssign};
use super::Polynomial;

fn divide_by_scalar_in_place(poly: &mut Polynomial, scalar: f64) {
    for (_, coefficient) in poly.coefficients.iter_mut() {
        *coefficient /= scalar;
    }
}

impl Div<f64> for Polynomial {
    type Output = Polynomial;

    fn div(mut self, rhs: f64) -> Self::Output {
        divide_by_scalar_in_place(&mut self, rhs);
        self
    }
}

impl Div<i32> for Polynomial {
    type Output = Polynomial;

    fn div(mut self, rhs: i32) -> Self::Output {
        divide_by_scalar_in_place(&mut self, rhs as f64);
        self
    }
}

impl DivAssign<f64> for Polynomial {
    fn div_assign(&mut self, rhs: f64) {
        divide_by_scalar_in_place(self, rhs);
    }
}

impl DivAssign<i32> for Polynomial {
    fn div_assign(&mut self, rhs: i32) {
        divide_by_scalar_in_place(self, rhs as f64);
    }
}

impl Rem<&Self> for Polynomial {
    type Output = Polynomial;

    fn rem(mut self, rhs: &Self) -> Self::Output {
        divide_in_place(&mut self, rhs);
        self
    }
}

impl RemAssign<&Self> for Polynomial {
    fn rem_assign(&mut self, rhs: &Self) {
        divide_in_place(self, rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn div_float() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly_divided_by_two = poly / 2.0;
        assert_eq!(vec![0.5, 1.0, -1.5], poly_divided_by_two.get_coefficients());
    }

    #[test]
    fn div_int() {
        let poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly_divided_by_two = poly / 2;
        assert_eq!(vec![0.5, 1.0, -1.5], poly_divided_by_two.get_coefficients());
    }

    #[test]
    fn div_assign_float() {
        let mut poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        poly /= 2.0;
        assert_eq!(vec![0.5, 1.0, -1.5], poly.get_coefficients());
    }

    #[test]
    fn div_assign_int() {
        let mut poly = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        poly /= 2;
        assert_eq!(vec![0.5, 1.0, -1.5], poly.get_coefficients());
    }
}