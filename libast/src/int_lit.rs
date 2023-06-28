#[derive(Debug, PartialEq)]
pub struct IntLit(String);

impl IntLit {
    pub fn new(val: impl Into<String>) -> Self {
        Self(val.into())
    }
}