use crate::{scanner::{Parse, Scanner}, var_decl::VarDecl};

#[derive(Debug, PartialEq)]
pub enum Stmt {
    VarDecl(VarDecl),
}

impl Parse for Stmt {
    type Item = Self;
    
    fn parse(scn: &mut Scanner) -> Option<Stmt> {
        VarDecl::parse(scn).map(Stmt::VarDecl)
    }
}