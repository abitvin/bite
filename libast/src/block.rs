use crate::stmt::Stmt;

#[derive(Debug, PartialEq)]
pub struct Block(Vec<Stmt>);

impl From<Vec<Stmt>> for Block {
    fn from(val: Vec<Stmt>) -> Self {
        Block(val)
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self(stmts)
    }
}