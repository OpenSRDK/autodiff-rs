use crate::Dual;

impl Dual<f64> {
    pub fn powi(self, n: i32) -> Self {
        Self::new(self.re.powi(n), n as f64 * self.re.powi(n - 1) * self.du)
    }

    pub fn powf(self, n: f64) -> Self {
        Self::new(self.re.powf(n), n * self.re.powf(n - 1.0) * self.du)
    }
}
