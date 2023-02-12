use std::{collections::HashSet, fmt::Debug};

use opensrdk_symbolic_computation::Expression;
use serde::{Deserialize, Serialize};

pub trait ContinuousDistribution: Clone + Debug + Serialize {
    fn value_ids(&self) -> HashSet<&str>;

    fn conditions(&self) -> Vec<&Expression>;

    fn condition_ids(&self) -> HashSet<&str> {
        self.conditions()
            .iter()
            .map(|v| v.symbols())
            .flatten()
            .collect()
    }

    fn pdf(&self) -> Expression;

    fn ln_pdf(&self) -> Expression {
        self.pdf().ln()
    }
}

pub type VariableId = String;
