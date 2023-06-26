#[derive(Debug, PartialEq)]
pub struct VarDecl {
    id: String,
    typ: Option<String>,
    expr: String,
}

impl VarDecl {
    pub fn new(id: impl Into<String>, typ: Option<String>, expr: impl Into<String>) -> Self {
        Self { id: id.into(), typ, expr: expr.into() }
    }
}