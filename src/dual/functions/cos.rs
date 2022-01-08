use crate::Dual;

impl Dual<f64> {
    pub fn cos(self) -> Self {
        Self::new(self.re.cos(), -self.du * self.re.sin())
    }
}
