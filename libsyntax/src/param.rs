use crate::{common::parse_two_ids, scanner::{Scanner, Parse}};

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
}

impl Parse for Param {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<Self> {
        parse_two_ids(scn).map(|(id, typ)| Self { id, typ })
    }
}