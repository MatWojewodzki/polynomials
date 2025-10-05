use std::ops::Neg;
use super::Polynomial;

impl Neg for Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Self::Output {
        self * -1
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