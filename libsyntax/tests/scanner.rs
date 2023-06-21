extern crate libsyntax;
use libsyntax::scanner::Scanner;

#[test]
fn scan_alphabetic() {
    let s = "azAZ";
    let mut c = Scanner::new(s);

    assert_eq!(c.scan_alphabetic(), Some('a'));
    assert_eq!(c.scan_alphabetic(), Some('z'));
    assert_eq!(c.scan_alphabetic(), Some('A'));
    assert_eq!(c.scan_alphabetic(), Some('Z'));
    assert_eq!(c.scan_alphabetic(), None);
}

#[test]
fn scan_alphanumeric() {
    let s = "azAZ123";
    let mut c = Scanner::new(s);

    assert_eq!(c.scan_alphanumeric(), Some('a'));
    assert_eq!(c.scan_alphanumeric(), Some('z'));
    assert_eq!(c.scan_alphanumeric(), Some('A'));
    assert_eq!(c.scan_alphanumeric(), Some('Z'));
    assert_eq!(c.scan_alphanumeric(), Some('1'));
    assert_eq!(c.scan_alphanumeric(), Some('2'));
    assert_eq!(c.scan_alphanumeric(), Some('3'));
    assert_eq!(c.scan_alphanumeric(), None);
}

#[test]
fn scan_is_digit() {
    let s = "139";
    let mut c = Scanner::new(s);

    assert_eq!(c.scan_digit(), Some('1'));
    assert_eq!(c.scan_digit(), Some('3'));
    assert_eq!(c.scan_digit(), Some('9'));
    assert_eq!(c.scan_digit(), None);
}

#[test]
fn scan_progress() {
    let s = "12hamBob";
    let mut c = Scanner::new(s);

    assert_eq!(c.scan_alphabetic(), None);
    assert_eq!(c.scan_alphabetic(), None);
    assert_eq!(c.scan_digit(), Some('1'));
    assert_eq!(c.scan_digit(), Some('2'));
    assert_eq!(c.scan("bam"), None);
    assert_eq!(c.scan("ham"), Some(String::from("ham")));
    assert_eq!(c.scan_digit(), None);
    assert_eq!(c.scan_alphanumeric(), Some('B'));
    assert_eq!(c.scan_alphanumeric(), Some('o'));
    assert_eq!(c.scan_digit(), None);
    assert_eq!(c.scan_alphabetic(), Some('b'));
    assert_eq!(c.scan_digit(), None);
}

#[test]
fn scan_text() {
    let s = "cheese+ham";
    let mut c = Scanner::new(s);

    assert_eq!(c.scan("cheese"), Some(String::from("cheese")));
    assert_eq!(c.scan(""), Some(String::from("")));
    assert_eq!(c.scan("+"), Some(String::from("+")));
    assert_eq!(c.scan("nope"), None);
    assert_eq!(c.scan("ham"), Some(String::from("ham")));
    assert_eq!(c.scan(""), Some(String::from("")));
    assert_eq!(c.scan(""), Some(String::from("")));
    assert_eq!(c.scan("nope again"), None);
}