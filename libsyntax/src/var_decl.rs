use crate::{scanner::Scanner, common::{parse_id, parse_int}};

#[derive(Debug, PartialEq)]
pub struct VarDecl {
    id: String,
    typ: Option<String>,
    expr: String,
}

impl VarDecl {
    pub fn new(id: impl Into<String>, typ: Option<String>, expr: impl Into<String>) -> Self {
        Self { id: id.into(), typ, expr: expr.into() }
    }

    pub fn parse(scn: &mut Scanner) -> Option<Self> {
        let id = parse_id(scn)?;
        scn.skip_spaces();

        let typ = if scn.skip(':') {
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

        Some(Self { id, typ, expr })
    }
}