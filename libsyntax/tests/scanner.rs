extern crate libsyntax;
use libsyntax::scanner::Scanner;

#[test]
fn scan_alphabetic() {
    let s = "azAZ";
    let mut scn = Scanner::new(s);

    assert_eq!(scn.scan_alphabetic(), Some('a'));
    assert_eq!(scn.scan_alphabetic(), Some('z'));
    assert_eq!(scn.scan_alphabetic(), Some('A'));
    assert_eq!(scn.scan_alphabetic(), Some('Z'));
    assert_eq!(scn.scan_alphabetic(), None);
}

#[test]
fn scan_alphanumeric() {
    let s = "azAZ123";
    let mut scn = Scanner::new(s);

    assert_eq!(scn.scan_alphanumeric(), Some('a'));
    assert_eq!(scn.scan_alphanumeric(), Some('z'));
    assert_eq!(scn.scan_alphanumeric(), Some('A'));
    assert_eq!(scn.scan_alphanumeric(), Some('Z'));
    assert_eq!(scn.scan_alphanumeric(), Some('1'));
    assert_eq!(scn.scan_alphanumeric(), Some('2'));
    assert_eq!(scn.scan_alphanumeric(), Some('3'));
    assert_eq!(scn.scan_alphanumeric(), None);
}

#[test]
fn scan_digit() {
    let s = "139";
    let mut scn = Scanner::new(s);

    assert_eq!(scn.scan_digit(), Some('1'));
    assert_eq!(scn.scan_digit(), Some('3'));
    assert_eq!(scn.scan_digit(), Some('9'));
    assert_eq!(scn.scan_digit(), None);
}

#[test]
fn scan_digits() {
    let s = "7";
    let mut scn = Scanner::new(s);

    assert_eq!(scn.scan_digits(), Some(String::from("7")));
    assert_eq!(scn.scan_digits(), None);

    let s = "139";
    let mut scn = Scanner::new(s);

    assert_eq!(scn.scan_digits(), Some(String::from("139")));
    assert_eq!(scn.scan_digits(), None);

    let s = "a";
    let mut scn = Scanner::new(s);

    assert_eq!(scn.scan_digits(), None);
}

#[test]
fn scan_progress() {
    let s = "12hamBob";
    let mut scn = Scanner::new(s);

    assert_eq!(scn.scan_alphabetic(), None);
    assert_eq!(scn.scan_alphabetic(), None);
    assert_eq!(scn.scan_digit(), Some('1'));
    assert_eq!(scn.scan_digit(), Some('2'));
    assert_eq!(scn.scan("bam"), None);
    assert_eq!(scn.scan("ham"), Some(String::from("ham")));
    assert_eq!(scn.scan_digit(), None);
    assert_eq!(scn.scan_alphanumeric(), Some('B'));
    assert_eq!(scn.scan_alphanumeric(), Some('o'));
    assert_eq!(scn.scan_digit(), None);
    assert_eq!(scn.scan_alphabetic(), Some('b'));
    assert_eq!(scn.scan_digit(), None);
}

#[test]
fn scan_spaces() {
    let s = "";
    let mut scn = Scanner::new(s);
    assert!(!scn.scan_spaces());

    let s = " ";
    let mut scn = Scanner::new(s);
    assert!(scn.scan_spaces());
}

#[test]
fn scan_text() {
    let s = "cheese+ham";
    let mut scn = Scanner::new(s);

    assert_eq!(scn.scan("cheese"), Some(String::from("cheese")));
    assert_eq!(scn.scan(""), Some(String::from("")));
    assert_eq!(scn.scan("+"), Some(String::from("+")));
    assert_eq!(scn.scan("nope"), None);
    assert_eq!(scn.scan("ham"), Some(String::from("ham")));
    assert_eq!(scn.scan(""), Some(String::from("")));
    assert_eq!(scn.scan(""), Some(String::from("")));
    assert_eq!(scn.scan("nope again"), None);
}