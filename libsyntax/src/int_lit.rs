use libast::int_lit::IntLit;
use crate::{common::parse_int, scanner::{Parse, Scanner}};

impl Parse for IntLit {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<IntLit> {
        parse_int(scn).map(|b| IntLit::new(b))
    }
}