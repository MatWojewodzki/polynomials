use super::super::Polynomial;
use num::Num;
use std::ops::{Add, AddAssign};

fn add_in_place<T>(poly1: &mut Polynomial<T>, poly2: &Polynomial<T>)
where
    T: Num + Clone,
{
    for (power, coefficient) in poly2.coefficients.iter() {
        poly1.add_coefficient_at(*power, coefficient.clone());
    }
}

impl<T> Add<&Self> for Polynomial<T>
where
    T: Num + Clone,
{
    type Output = Polynomial<T>;

    fn add(mut self, rhs: &Self) -> Self::Output {
        add_in_place(&mut self, rhs);
        self
    }
}

impl<T> Add<T> for Polynomial<T>
where
    T: Num + Clone,
{
    type Output = Polynomial<T>;

    fn add(mut self, rhs: T) -> Self::Output {
        self.add_coefficient_at(0, rhs);
        self
    }
}

impl<T> AddAssign<&Self> for Polynomial<T>
where
    T: Num + Clone,
{
    fn add_assign(&mut self, rhs: &Self) {
        add_in_place(self, rhs);
    }
}

impl<T> AddAssign<T> for Polynomial<T>
where
    T: Num + Clone,
{
    fn add_assign(&mut self, rhs: T) {
        self.add_coefficient_at(0, rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn add() {
        let poly1 = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly2 = Polynomial::from_coefficients(&vec![-2.0, -2.0, -1.0]);
        let poly3 = poly1 + &poly2;
        assert_eq!(vec![-1.0, 0.0, -4.0], poly3.get_coefficients());
    }

    #[test]
    fn add_scalar() {
        let poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        let poly_plus_two = poly + 2.0;
        assert_eq!(vec![-2.0, 0.0, 3.0], poly_plus_two.get_coefficients());
    }

    #[test]
    fn add_assign() {
        let mut poly1 = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly2 = Polynomial::from_coefficients(&vec![-2.0, -2.0, -1.0]);
        poly1 += &poly2;
        assert_eq!(vec![-1.0, 0.0, -4.0], poly1.get_coefficients());
    }

    #[test]
    fn add_assign_scalar() {
        let mut poly = Polynomial::from_coefficients(&vec![-2.0, 0.0, 1.0]);
        poly += 2.0;
        assert_eq!(vec![-2.0, 0.0, 3.0], poly.get_coefficients());
    }
}
