use crate::{param::Param, scanner::Scanner, common::parse_id};

#[derive(Debug, PartialEq)]
pub struct FnDecl {
    id: String,
    params: Vec<Param>,
    ret_type: String,
}

impl FnDecl {
    pub fn new(id: impl Into<String>, params: Vec<Param>, ret_type: impl Into<String>) -> Self {
        Self { id: id.into(), params, ret_type: ret_type.into() }
    }

    pub fn parse(mut scn: &mut Scanner) -> Option<Self> {
        let id = parse_id(&mut scn)?;
        scn.scan_spaces();
        scn.scan(":")?;
        scn.scan_spaces();
        let params = parse_params(&mut scn)?;
        scn.scan_spaces();
        scn.scan("->")?;
        scn.scan_spaces();
        let ret_type = parse_id(&mut scn)?;
        scn.scan_spaces();
        scn.skip_newline();
        scn.scan(".")?;
        scn.scan_spaces();
        scn.skip_newline();

        Some(Self { id, params, ret_type })
    }

}

fn parse_params(mut scn: &mut Scanner) -> Option<Vec<Param>> {
    scn.scan("(")?;
    scn.scan_spaces();

    let mut params = Param::parse(scn)
        .map(|x| vec![x])?;

    loop {
        scn.scan_spaces();
        
        if !scn.skip(',') {
            break;
        }

        scn.scan_spaces();
        params.push(Param::parse(scn)?);
    }

    scn.scan_spaces();
    scn.scan(")")?;

    Some(params)
}