extern crate libsyntax;
use libsyntax::scanner::Scanner;

#[test]
fn scan_alphabetic() {
    let s = "19aZ";
    let mut c = Scanner::new(s);

    assert_eq!(c.scan_alphabetic(), None);
    assert_eq!(c.scan_alphabetic(), None);
    assert_eq!(c.scan_alphabetic(), Some('a'));
    assert_eq!(c.scan_alphabetic(), Some('Z'));
}

#[test]
fn scan_is_digit() {
    let s = "19aZ";
    let mut c = Scanner::new(s);

    assert_eq!(c.scan_digit(), Some('1'));
    assert_eq!(c.scan_digit(), Some('9'));
    assert_eq!(c.scan_digit(), None);
    assert_eq!(c.scan_digit(), None);
}

#[test]
fn scan_text() {
    let s = "cheese+ham";
    let mut c = Scanner::new(s);

    assert_eq!(c.scan("cheese"), Some(String::from("cheese")));
    assert_eq!(c.scan("+"), Some(String::from("+")));
    assert_eq!(c.scan("ham"), Some(String::from("ham")));
}