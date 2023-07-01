use crate::{scanner::{Parse, Scanner}, common::{parse_id, parse_int}};
use libast::var_decl::VarDecl;

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
        
        scn.scan("=");
        scn.skip_spaces();
        let expr = parse_int(scn)?;

        Some(Self::new(id, typ, expr))
    }
}