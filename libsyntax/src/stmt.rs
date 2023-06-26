use crate::scanner::{Parse, Scanner};
use libast::{stmt::Stmt, var_decl::VarDecl};


impl Parse for Stmt {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<Stmt> {
        VarDecl::parse(scn).map(Stmt::VarDecl)
    }
}