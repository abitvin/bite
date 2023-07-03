use crate::{block::Block, expr::Expr, r#if::If, var_decl::VarDecl};

#[derive(Debug, PartialEq)]
pub enum Stmt {
    If(If),
    VarDecl(VarDecl),
}

impl Stmt {
    pub fn new_if(if_block: (bool, impl Into<Block>), elif_blocks: Vec<(bool, Block)>, else_block: impl Into<Option<Block>>) -> Self {
        Self::If(If::new(if_block, elif_blocks, else_block))
    }

    pub fn new_var_decl(id: impl Into<String>, typ: Option<String>, expr: Expr) -> Self {
        Self::VarDecl(VarDecl::new(id, typ, expr))
    }
}