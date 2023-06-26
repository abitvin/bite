use crate::var_decl::VarDecl;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    VarDecl(VarDecl),
}