use libast::bool_lit::BoolLit;
use libsyntax::scanner::{Parse, Scanner};

#[test]
fn parse_false() {
    let s = "false";
    let mut scn = Scanner::new(s);
    assert_eq!(BoolLit::parse(&mut scn), Some(BoolLit::new(false)));
}

#[test]
fn parse_true() {
    let s = "true";
    let mut scn = Scanner::new(s);
    assert_eq!(BoolLit::parse(&mut scn), Some(BoolLit::new(true)));
}

#[test]
fn parse_bool_error() {
    let s = "nope";
    let mut scn = Scanner::new(s);
    assert_eq!(BoolLit::parse(&mut scn), None);
}