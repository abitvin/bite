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

    pub fn parse(scn: &mut Scanner) -> Option<Self> {
        let id = parse_id(scn)?;
        scn.skip_spaces();
        scn.scan(":")?;
        scn.skip_spaces();
        let params = parse_params(scn)?;
        scn.skip_spaces();
        scn.scan("->")?;
        scn.skip_spaces();
        let ret_type = parse_id(scn)?;
        scn.skip_spaces();
        scn.skip_newline();
        scn.scan(".")?;
        scn.skip_spaces();
        scn.skip_newline();

        Some(Self { id, params, ret_type })
    }

}

fn parse_params(scn: &mut Scanner) -> Option<Vec<Param>> {
    scn.scan("(")?;
    scn.skip_spaces();

    let mut params = Param::parse(scn)
        .map(|x| vec![x])?;

    loop {
        scn.skip_spaces();
        
        if !scn.skip(',') {
            break;
        }

        scn.skip_spaces();
        params.push(Param::parse(scn)?);
    }

    scn.skip_spaces();
    scn.scan(")")?;

    Some(params)
}