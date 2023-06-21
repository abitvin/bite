extern crate libsyntax;
use libsyntax::{common::parse_id, scanner::Scanner};

#[test]
fn test_parse_id() {
    let id = "a";
    let mut s = Scanner::new(id);
    assert_eq!(parse_id(&mut s), Some(String::from("a")));

    let id = "";
    let mut s = Scanner::new(id);
    assert_eq!(parse_id(&mut s), None);

    let id = "monkey";
    let mut s = Scanner::new(id);
    assert_eq!(parse_id(&mut s), Some(String::from("monkey")));

    let id = "monkey123";
    let mut s = Scanner::new(id);
    assert_eq!(parse_id(&mut s), Some(String::from("monkey123")));

    let id = "123";
    let mut s = Scanner::new(id);
    assert_eq!(parse_id(&mut s), None);
}