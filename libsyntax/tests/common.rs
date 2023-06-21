extern crate libsyntax;
use libsyntax::{common::{parse_id, parse_int}, scanner::Scanner};

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

#[test]
fn test_parse_int() {
    let i = "";
    let mut s = Scanner::new(i);
    assert_eq!(parse_int(&mut s), None);

    let i = "-";
    let mut s = Scanner::new(i);
    assert_eq!(parse_int(&mut s), None);

    let i = "a";
    let mut s = Scanner::new(i);
    assert_eq!(parse_int(&mut s), None);

    let i = "1";
    let mut s = Scanner::new(i);
    assert_eq!(parse_int(&mut s), Some(String::from("1")));

    let i = "123456789012345678901234567890123456789012345678901234567890";
    let mut s = Scanner::new(i);
    assert_eq!(parse_int(&mut s), Some(String::from("123456789012345678901234567890123456789012345678901234567890")));

    let i = "-1";
    let mut s = Scanner::new(i);
    assert_eq!(parse_int(&mut s), Some(String::from("-1")));
}