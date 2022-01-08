use crate::Dual;

impl Dual<f64> {
    pub fn ln(self) -> Self {
        Self::new(self.re.ln(), self.du / self.re)
    }
}
