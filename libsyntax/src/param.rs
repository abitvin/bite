use crate::{common::parse_two_ids, scanner::{Scanner, Parse}};
use libast::param::Param;

impl Parse for Param {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<Self> {
        parse_two_ids(scn).map(|(id, typ)| Self { id, typ })
    }
}