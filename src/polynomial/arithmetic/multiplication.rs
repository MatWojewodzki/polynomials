use std::ops::{Mul, MulAssign};
use num::Num;
use super::Polynomial;

fn multiply<T>(poly1: &Polynomial<T>, poly2: &Polynomial<T>) -> Polynomial<T>
where
    T: Num + Clone + Mul<Output = T>
{
    let mut poly = Polynomial::zero();
    for (power, coefficient) in poly1.coefficients.iter() {
        for (other_power, other_coefficient) in poly2.coefficients.iter() {
            poly.add_coefficient_at(
                *power + *other_power,
                coefficient.clone() * other_coefficient.clone()
            );
        }
    }
    poly
}

fn multiply_in_place_by_scalar<T>(poly: &mut Polynomial<T>, scalar: T)
where
    T: Num + Clone 
{
    // Prevent zeros from being present in the map
    if scalar == T::zero() {
        *poly = Polynomial::zero();
        return;
    }
    for (_, coefficient) in poly.coefficients.iter_mut() {
        *coefficient = coefficient.clone() * scalar.clone();
    }
}

impl<T> Mul<&Self> for Polynomial<T>
where
    T: Num + Clone
{
    type Output = Polynomial<T>;

    fn mul(self, rhs: &Self) -> Self::Output {
        multiply(&self, rhs)
    }
}

impl<T> Mul<T> for Polynomial<T>
where
    T: Num + Clone 
{
    type Output = Polynomial<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        multiply_in_place_by_scalar(&mut self, rhs);
        self
    }
}

impl<T> MulAssign<&Self> for Polynomial<T>
where
    T: Num + Clone
{
    fn mul_assign(&mut self, rhs: &Self) {
        *self = multiply(&self, rhs);
    }
}

impl<T> MulAssign<T> for Polynomial<T>
where
    T: Num + Clone 
{
    fn mul_assign(&mut self, rhs: T) {
        multiply_in_place_by_scalar(self, rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn mul() {
        let poly1 = Polynomial::from_coefficients(&vec![1.0, -2.0]);
        let poly2 = Polynomial::from_coefficients(&vec![-2.0, 0.0, 3.0]);
        let poly3 = poly1 * &poly2;
        assert_eq!(vec![-2.0, 4.0, 3.0, -6.0], poly3.get_coefficients());
    }

    #[test]
    fn mul_scalar() {
        let poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        let poly_times_two = poly * 2.0;
        assert_eq!(vec![-4.0, 0.0, 2.0], poly_times_two.get_coefficients());
    }

    #[test]
    fn mul_assign() {
        let mut poly1 = Polynomial::from_coefficients(&vec![1.0, -2.0]);
        let poly2 = Polynomial::from_coefficients(&vec![-2.0, 0.0, 3.0]);
        poly1 *= &poly2;
        assert_eq!(vec![-2.0, 4.0, 3.0, -6.0], poly1.get_coefficients());
    }

    #[test]
    fn mul_assign_scalar() {
        let mut poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        poly *= 2.0;
        assert_eq!(vec![-4.0, 0.0, 2.0], poly.get_coefficients());
    }

    /// Check whether the polynomial is normalised correctly after the multiplication
    #[test]
    fn mul_by_scalar_zero() {
        let poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        let poly_times_zero = poly * 0.0;
        assert_eq!(Polynomial::zero(), poly_times_zero);
    }
}