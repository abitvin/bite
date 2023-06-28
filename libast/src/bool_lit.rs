#[derive(Debug, PartialEq)]
pub struct BoolLit(bool);

impl BoolLit {
    pub fn new(val: bool) -> Self {
        Self(val)
    }
}