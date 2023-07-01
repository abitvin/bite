use crate::{common::try_parse, scanner::{Parse, Scanner}};
use libast::{r#if::If, stmt::Stmt, var_decl::VarDecl};

impl Parse for Stmt {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<Self> {
        try_parse(scn, |s| If::parse(s).map(Self::If))
        .or(try_parse(scn, |s| VarDecl::parse(s).map(Self::VarDecl)))
    }
}