use crate::{stmt::Stmt, scanner::Scanner};

#[derive(Debug, PartialEq)]
pub struct Block(Vec<Stmt>);

impl From<Vec<Stmt>> for Block {
    fn from(val: Vec<Stmt>) -> Self {
        Block(val)
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl Block {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn parse(scn: &mut Scanner) -> Option<Block> {
        let mut stmts = vec![];

        loop {
            let mut try_scn = scn.clone();
            let stmt = parse_stmt(&mut try_scn);

            match stmt {
                Some(stmt) => { stmts.push(stmt); scn.replace(try_scn); }
                None => break,
            }
        }

        Some(Block(stmts))
    }
}

fn parse_stmt(scn: &mut Scanner) -> Option<Stmt> {
    scn.skip_whitespaces();
    let stmt = Stmt::parse(scn)?;
    scn.skip_whitespaces();
    scn.skip_newline();
    
    Some(stmt)
}