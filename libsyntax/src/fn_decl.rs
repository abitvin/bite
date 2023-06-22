use crate::{common::parse_id, param::Param, stmt::Stmt, scanner::Scanner};

#[derive(Debug, PartialEq)]
pub struct FnDecl {
    id: String,
    params: Vec<Param>,
    ret_type: String,
    stmts: Vec<Stmt>,
}

impl FnDecl {
    pub fn new(id: impl Into<String>, params: Vec<Param>, ret_type: impl Into<String>, stmts: Option<Vec<Stmt>>) -> Self {
        Self { 
            id: id.into(), 
            params, ret_type: 
            ret_type.into(), 
            stmts: stmts.unwrap_or_default(),
        }
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

        let mut stmts = vec![];

        loop {
            let mut try_scn = scn.clone();
            let stmt = parse_stmt(&mut try_scn);

            match stmt {
                Some(stmt) => { stmts.push(stmt); scn.replace(try_scn); }
                None => break,
            }
        }

        scn.skip_spaces();
        scn.scan(".")?;
        
        Some(Self { id, params, ret_type, stmts })
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

fn parse_stmt(scn: &mut Scanner) -> Option<Stmt> {
    scn.skip_whitespaces();
    let stmt = Stmt::parse(scn)?;
    scn.skip_whitespaces();
    scn.skip_newline();
    
    Some(stmt)
}