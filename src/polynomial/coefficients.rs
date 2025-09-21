use super::Polynomial;

impl Polynomial {
    pub fn set_coefficient_at(&mut self, power: u32, coefficient: f64) {
        if coefficient == 0.0 {
            self.coefficients.remove(&power);
            return;
        }
        self.coefficients.insert(power, coefficient);
    }

    pub fn get_coefficient_at(&self, power: u32) -> f64 {
        self.coefficients.get(&power).copied().unwrap_or(0.0)
    }

    pub fn add_coefficient_at(&mut self, power: u32, coefficient: f64) {
        self.set_coefficient_at(power, self.get_coefficient_at(power) + coefficient);
    }

    pub fn sub_coefficient_at(&mut self, power: u32, coefficient: f64) {
        self.set_coefficient_at(power, self.get_coefficient_at(power) - coefficient);
    }

    pub fn mul_coefficient_at(&mut self, power: u32, coefficient: f64) {
        self.set_coefficient_at(power, self.get_coefficient_at(power) * coefficient);
    }
}