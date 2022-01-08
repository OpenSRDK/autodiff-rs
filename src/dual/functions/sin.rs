use crate::Dual;

impl Dual<f64> {
    pub fn sin(self) -> Self {
        Self::new(self.re.sin(), self.du * self.re.cos())
    }
}
