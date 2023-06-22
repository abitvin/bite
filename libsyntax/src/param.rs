use crate::{scanner::Scanner, common::parse_id};

#[derive(Debug, PartialEq)]
pub struct Param {
    pub id: String,
    pub typ: String,
}

impl Param {
    pub fn new(id: &str, typ: &str) -> Self {
        Self { 
            id: String::from(id), 
            typ: String::from(typ),
        }
    }

    pub fn parse(mut scn: &mut Scanner) -> Option<Self> {
        let id = parse_id(&mut scn)?;
        scn.skip_spaces();
        scn.scan(":")?;
        scn.skip_spaces();
        let typ = parse_id(&mut scn)?;
        
        Some(Self { id, typ })
    }
}