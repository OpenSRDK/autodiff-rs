use crate::Dual;

impl Dual<f64> {
    pub fn exp(self) -> Self {
        Self::new(self.re.exp(), self.du * self.re.exp())
    }
}
