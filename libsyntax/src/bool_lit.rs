use libast::bool_lit::BoolLit;
use crate::{common::parse_bool, scanner::{Parse, Scanner}};

impl Parse for BoolLit {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<BoolLit> {
        parse_bool(scn).map(|b| BoolLit::new(b))
    }
}