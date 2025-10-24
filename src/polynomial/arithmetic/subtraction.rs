use std::ops::{Sub, SubAssign};
use num::Num;
use super::Polynomial;

fn subtract_in_place<T>(poly1: &mut Polynomial<T>, poly2: &Polynomial<T>)
where
    T: Num + Clone
{
    for (power, coefficient) in poly2.coefficients.iter() {
        poly1.sub_coefficient_at(*power, coefficient.clone());
    }
}

impl<T> Sub<&Self> for Polynomial<T>
where
    T: Num + Clone
{
    type Output = Polynomial<T>;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        subtract_in_place(&mut self, rhs);
        self
    }
}

impl<T> Sub<T> for Polynomial<T>
where
    T: Num + Clone
{
    type Output = Polynomial<T>;

    fn sub(mut self, rhs: T) -> Self::Output {
        self.sub_coefficient_at(0, rhs);
        self
    }
}

impl<T> SubAssign<&Self> for Polynomial<T>
where
    T: Num + Clone
{
    fn sub_assign(&mut self, rhs: &Self) {
        subtract_in_place(self, rhs);
    }
}

impl<T> SubAssign<T> for Polynomial<T>
where
    T: Num + Clone
{
    fn sub_assign(&mut self, rhs: T) {
        self.sub_coefficient_at(0, rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn sub() {
        let poly1 = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly2 = Polynomial::from_coefficients(&vec![-2.0, 2.0, -1.0]);
        let poly3 = poly1 - &poly2;
        assert_eq!(vec![3.0, 0.0, -2.0], poly3.get_coefficients());
    }

    #[test]
    fn sub_scalar() {
        let poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        let poly_minus_two = poly - 2.0;
        assert_eq!(vec![-2.0, 0.0, -1.0], poly_minus_two.get_coefficients());
    }

    #[test]
    fn sub_assign() {
        let mut poly1 = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly2 = Polynomial::from_coefficients(&vec![-2.0, 2.0, -1.0]);
        poly1 -= &poly2;
        assert_eq!(vec![3.0, 0.0, -2.0], poly1.get_coefficients());
    }

    #[test]
    fn sub_assign_scalar() {
        let mut poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        poly -= 2.0;
        assert_eq!(vec![-2.0, 0.0, -1.0], poly.get_coefficients());
    }
}