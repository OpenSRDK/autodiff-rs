#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    id: u32,
}

impl Symbol {
    pub(crate) fn new(id: u32) -> Self {
        Self { id }
    }
}
