use std::ops::{Mul, MulAssign};
use super::Polynomial;

fn multiply(poly1: &Polynomial, poly2: &Polynomial) -> Polynomial {
    let mut poly = Polynomial::zero();
    for (power, coefficient) in poly1.coefficients.iter() {
        for (other_power, other_coefficient) in poly2.coefficients.iter() {
            poly.add_coefficient_at(
                *power + *other_power,
                *coefficient * *other_coefficient
            );
        }
    }
    poly
}

fn multiply_in_place_by_scalar(poly: &mut Polynomial, scalar: f64) {
    for (_, coefficient) in poly.coefficients.iter_mut() {
        *coefficient *= scalar;
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, other: Self) -> Self::Output {
        multiply(&self, &other)
    }
}

impl Mul<f64> for Polynomial {
    type Output = Polynomial;

    fn mul(mut self, other: f64) -> Self::Output {
        multiply_in_place_by_scalar(&mut self, other);
        self
    }
}

impl Mul<i32> for Polynomial {
    type Output = Polynomial;

    fn mul(mut self, other: i32) -> Self::Output {
        multiply_in_place_by_scalar(&mut self, other as f64);
        self
    }
}

impl MulAssign for Polynomial {
    fn mul_assign(&mut self, other: Self) {
        *self = multiply(&self, &other);
    }
}

impl MulAssign<f64> for Polynomial {
    fn mul_assign(&mut self, other: f64) {
        multiply_in_place_by_scalar(self, other);
    }
}

impl MulAssign<i32> for Polynomial {
    fn mul_assign(&mut self, other: i32) {
        multiply_in_place_by_scalar(self, other as f64);
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn mul() {
        let poly1 = Polynomial::from_coefficients(&vec![1.0, -2.0]);
        let poly2 = Polynomial::from_coefficients(&vec![-2.0, 0.0, 3.0]);
        let poly3 = poly1 * poly2;
        assert_eq!(vec![-2.0, 4.0, 3.0, -6.0], poly3.get_coefficients());
    }

    #[test]
    fn mul_float() {
        let poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        let poly_times_two = poly * 2.0;
        assert_eq!(vec![-4.0, 0.0, 2.0], poly_times_two.get_coefficients());
    }

    #[test]
    fn mul_int() {
        let poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        let poly_times_two = poly * 2;
        assert_eq!(vec![-4.0, 0.0, 2.0], poly_times_two.get_coefficients());
    }

    #[test]
    fn mul_assign() {
        let mut poly1 = Polynomial::from_coefficients(&vec![1.0, -2.0]);
        let poly2 = Polynomial::from_coefficients(&vec![-2.0, 0.0, 3.0]);
        poly1 *= poly2;
        assert_eq!(vec![-2.0, 4.0, 3.0, -6.0], poly1.get_coefficients());
    }

    #[test]
    fn mul_assign_float() {
        let mut poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        poly *= 2.0;
        assert_eq!(vec![-4.0, 0.0, 2.0], poly.get_coefficients());
    }

    #[test]
    fn mul_assign_int() {
        let mut poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        poly *= 2;
        assert_eq!(vec![-4.0, 0.0, 2.0], poly.get_coefficients());
    }
}