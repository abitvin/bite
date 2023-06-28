use libast::int_lit::IntLit;
use libsyntax::scanner::{Parse, Scanner};

#[test]
fn parse_zero_int() {
    let s = "0";
    let mut scn = Scanner::new(s);
    assert_eq!(IntLit::parse(&mut scn), Some(IntLit::new("0")));
}

#[test]
fn parse_negative_int() {
    let s = "-1";
    let mut scn = Scanner::new(s);
    assert_eq!(IntLit::parse(&mut scn), Some(IntLit::new("-1")));
}

#[test]
fn parse_positive_int() {
    let s = "2";
    let mut scn = Scanner::new(s);
    assert_eq!(IntLit::parse(&mut scn), Some(IntLit::new("2")));
}

#[test]
fn parse_big_int() {
    let s = "99999999999999999999999999999999999999999999999";
    let mut scn = Scanner::new(s);
    assert_eq!(IntLit::parse(&mut scn), Some(IntLit::new("99999999999999999999999999999999999999999999999")));
}

#[test]
fn parse_int_error() {
    let s = "nah";
    let mut scn = Scanner::new(s);
    assert_eq!(IntLit::parse(&mut scn), None);
}