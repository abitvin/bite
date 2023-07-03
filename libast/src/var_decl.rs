use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub struct VarDecl {
    id: String,
    typ: Option<String>,
    expr: Expr,
}

impl VarDecl {
    pub fn new(id: impl Into<String>, typ: Option<String>, expr: Expr) -> Self {
        Self { id: id.into(), typ, expr }
    }
}