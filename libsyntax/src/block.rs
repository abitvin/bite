use crate::{stmt::Stmt, scanner::Scanner};

#[derive(Debug, PartialEq)]
pub struct Block(Vec<Stmt>);

impl Into<Block> for Vec<Stmt> {
    fn into(self) -> Block {
        Block(self)
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