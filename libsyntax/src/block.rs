use crate::scanner::{Parse, Scanner};
use libast::{block::Block, stmt::Stmt};

impl Parse for Block {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<Block> {
        let mut stmts = vec![];

        loop {
            let mut try_scn = scn.clone();
            let stmt = parse_stmt(&mut try_scn);

            match stmt {
                Some(stmt) => { stmts.push(stmt); scn.replace(try_scn); }
                None => break,
            }
        }

        Some(Block::new(stmts))
    }
}

fn parse_stmt(scn: &mut Scanner) -> Option<Stmt> {
    scn.skip_whitespaces();
    let stmt = Stmt::parse(scn)?;
    scn.skip_whitespaces();
    scn.skip_newline();
    
    Some(stmt)
}