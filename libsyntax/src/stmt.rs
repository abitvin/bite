use crate::scanner::{Parse, Scanner};
use libast::{r#if::If, stmt::Stmt, var_decl::VarDecl};


impl Parse for Stmt {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<Stmt> {
        if let Some(stmt) = parse_stmt(scn, |s| If::parse(s).map(Stmt::If)) {
            Some(stmt)
        }
        else {
            parse_stmt(scn, |s| VarDecl::parse(s).map(Stmt::VarDecl))
        }
    }
}

fn parse_stmt(scn: &mut Scanner, f: impl Fn(&mut Scanner) -> Option<Stmt>) -> Option<Stmt> {
    let mut try_scn = scn.clone();

    if let Some(stmt) = f(&mut try_scn) {
        scn.replace(try_scn);
        Some(stmt)
    }
    else {
        None
    }
}