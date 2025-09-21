use std::ops::{Div, DivAssign};
use super::Polynomial;

fn divide_by_scalar_in_place(poly: &mut Polynomial, scalar: f64) {
    for (_, coefficient) in poly.coefficients.iter_mut() {
        *coefficient /= scalar;
    }
}

impl Div<f64> for Polynomial {
    type Output = Polynomial;

    fn div(mut self, other: f64) -> Self::Output {
        divide_by_scalar_in_place(&mut self, other);
        self
    }
}

impl DivAssign<f64> for Polynomial {
    fn div_assign(&mut self, other: f64) {
        divide_by_scalar_in_place(self, other);
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn polynomial_div_float() {
        let poly1 = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly2 = poly1 / 2.0;
        assert!(poly2 == Polynomial::from_coefficients(&vec![0.5, 1.0, -1.5]));
    }

    #[test]
    fn polynomial_div_assign_float() {
        let mut poly1 = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        poly1 /= 2.0;
        assert!(poly1 == Polynomial::from_coefficients(&vec![0.5, 1.0, -1.5]));
    }
}