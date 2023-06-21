extern crate libsyntax;
use libsyntax::{
    param::Param,
    scanner::Scanner,
};

#[test]
fn parse_param() {
    let s = "a : B";
    let mut scn = Scanner::new(s);
    assert_eq!(Param::parse(&mut scn), Some(Param::new("a", "B")));
}