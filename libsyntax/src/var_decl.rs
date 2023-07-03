use crate::{scanner::{Parse, Scanner}, common::parse_id};
use libast::{var_decl::VarDecl, expr::Expr};

impl Parse for VarDecl {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<Self> {
        let id = parse_id(scn)?;
        scn.skip_spaces();

        let typ = if scn.has(":") {
            scn.skip_spaces();
            let typ = Some(parse_id(scn)?);
            scn.skip_spaces();
            typ
        }
        else {
            None
        };
        
        scn.scan("=")?;
        scn.skip_spaces();
        let expr = Expr::parse(scn)?;

        Some(Self::new(id, typ, expr))
    }
}