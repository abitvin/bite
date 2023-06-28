use libast::{struct_decl::StructDecl, prop::Prop};
use libsyntax::scanner::{Parse, Scanner};

#[test]
fn parse_empty_struct() {
    let s = "Cheese : struct
                   .";

    let mut scn = Scanner::new(s);
    assert_eq!(StructDecl::parse(&mut scn), Some(StructDecl::new("Cheese", vec![])))
}

#[test]
fn parse_struct_with_props() {
    let s = "Ham : struct
                        a : i32
                        b : i8
                        c : bool
                   .";

    let props = vec![
        Prop::new("a", "i32"),
        Prop::new("b", "i8"),
        Prop::new("c", "bool"),
    ];
    let mut scn = Scanner::new(s);
    assert_eq!(StructDecl::parse(&mut scn), Some(StructDecl::new("Ham", props)))
}