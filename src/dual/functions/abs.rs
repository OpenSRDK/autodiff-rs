use crate::Dual;

impl Dual<f64> {
    pub fn abs(self) -> Self {
        Self::new(self.re.abs(), self.du * self.re.signum())
    }
}
