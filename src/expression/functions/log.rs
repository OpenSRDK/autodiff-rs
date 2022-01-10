use crate::Expression;

impl Expression {
    pub fn log(self, antilogarith: Expression) -> Self {
        if let Expression::Constant(v) = self {}

        if self.symbols().intersection(&antilogarith.symbols()).count() > 0 {
            panic!("Intersection of symbols of base and exponent must be empty set.");
        }

        Expression::Log(self.into(), antilogarith.into())
    }
}
