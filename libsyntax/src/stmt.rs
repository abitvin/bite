use crate::{scanner::Scanner, var_decl::VarDecl};

#[derive(Debug, PartialEq)]
pub enum Stmt {
    VarDecl(VarDecl),
}

impl Stmt {
    pub fn parse(scn: &mut Scanner) -> Option<Stmt> {
        VarDecl::parse(scn).map(|v| Stmt::VarDecl(v))
    }
}