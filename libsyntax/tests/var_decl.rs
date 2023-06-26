use libast::var_decl::VarDecl;
use libsyntax::scanner::{Parse, Scanner};

#[test]
fn var_decl() {
    let s = "a = 137";
    let mut scn = Scanner::new(s);
    assert_eq!(VarDecl::parse(&mut scn), Some(VarDecl::new("a", None, "137")));

    let s = "cheese : Cheese = 1234567";
    let mut scn = Scanner::new(s);
    assert_eq!(VarDecl::parse(&mut scn), Some(VarDecl::new("cheese", Some(String::from("Cheese")), "1234567")));
}