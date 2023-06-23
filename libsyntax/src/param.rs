use crate::{common::parse_two_ids, scanner::Scanner};

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

    pub fn parse(scn: &mut Scanner) -> Option<Self> {
        parse_two_ids(scn).map(|(id, typ)| Self { id, typ })
    }
}