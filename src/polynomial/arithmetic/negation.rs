use std::ops::Neg;
use super::Polynomial;

impl Neg for Polynomial {
    type Output = Polynomial;

    fn neg(mut self) -> Self::Output {
        for (_, coefficient) in self.coefficients.iter_mut() {
            *coefficient *= -1.0;
        }
        self
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