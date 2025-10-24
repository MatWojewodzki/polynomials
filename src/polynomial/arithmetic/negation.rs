use std::ops::Neg;
use num::Num;
use super::Polynomial;

impl<T> Neg for Polynomial<T>
where
    T: Num + Clone + Neg<Output = T>
{
    type Output = Polynomial<T>;

    fn neg(self) -> Self::Output {
        self * -T::one()
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn polynomial_negation() {
        let poly1 = Polynomial::from_coefficients(&vec![1.0, 2.0, -3.0]);
        let poly2 = Polynomial::from_coefficients(&vec![-1.0, -2.0, 3.0]);
        assert_eq!(poly1, -poly2);
    }
}