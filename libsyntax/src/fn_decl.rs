use crate::{common::parse_id, scanner::{Parse, Scanner}};
use libast::{block::Block, fn_decl::FnDecl, param::Param};

impl Parse for FnDecl {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<Self> {
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
        let block = Block::parse(scn)?;
        scn.skip_spaces();
        scn.scan(".")?;
        
        Some(Self::new(id, params, ret_type, block))
    }
}

fn parse_params(scn: &mut Scanner) -> Option<Vec<Param>> {
    scn.scan("(")?;
    scn.skip_spaces();

    let mut params = vec![];
    
    if let Some(param) = Param::parse(scn) {
        params.push(param);

        loop {
            scn.skip_spaces();
            
            if !scn.skip(',') {
                break;
            }
    
            scn.skip_spaces();
            params.push(Param::parse(scn)?);
        }
    }

    scn.skip_spaces();
    scn.scan(")")?;

    Some(params)
}