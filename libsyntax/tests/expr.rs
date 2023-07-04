use std::vec;

use libast::{expr::{BoolLit, Expr, IntLit, Var, StructLit, StructLitArg}, span::Span};
use libsyntax::scanner::{Parse, Scanner};

#[test]
fn parse_bool_false() {
    let s = "false";
    let mut scn = Scanner::new(s);
    assert_eq!(BoolLit::parse(&mut scn), Some(BoolLit::new(false)));
}

#[test]
fn parse_bool_true() {
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

#[test]
fn parse_expr_with_vars() {
    let expr = Expr::new_add(vec![
        Expr::new_var("a", Span::new(0, 0, 0, 1)),
        Expr::new_var("b", Span::new(4, 0, 4, 1)),
        Expr::new_var("c", Span::new(8, 0, 8, 1)),
    ]);
    let s = "a + b + c";
    let mut scn = Scanner::new(s);
    assert_eq!(Expr::parse(&mut scn), Some(expr));
}

#[test]
fn parse_add() {
    let expr = Expr::new_add(vec![
        Expr::new_int_lit("1"),
        Expr::new_int_lit("2"),
        Expr::new_int_lit("3"),
    ]);
    let s = "1 + 2 + 3";
    let mut scn = Scanner::new(s);
    assert_eq!(Expr::parse(&mut scn), Some(expr));
}

#[test]
fn parse_div() {
    let expr = Expr::new_div(vec![
        Expr::new_int_lit("3"),
        Expr::new_int_lit("4"),
        Expr::new_int_lit("5"),
    ]);
    let s = "3 / 4 / 5";
    let mut scn = Scanner::new(s);
    assert_eq!(Expr::parse(&mut scn), Some(expr));
}

#[test]
fn parse_grp() {
    let s = "()";
    let mut scn = Scanner::new(s);
    assert_eq!(Expr::parse(&mut scn), None);

    let expr = Expr::new_add(vec![
        Expr::new_grp(Expr::new_int_lit("3")),
        Expr::new_grp(Expr::new_sub(vec![
            Expr::new_int_lit("4"),
            Expr::new_int_lit("5"),
        ])),
        Expr::new_grp(Expr::new_grp(Expr::new_grp(Expr::new_int_lit("6")))),
    ]);
    let s = "(3) + (4 - 5) + (((6)))";
    let mut scn = Scanner::new(s);
    assert_eq!(Expr::parse(&mut scn), Some(expr));
}

#[test]
fn parse_mul() {
    let expr = Expr::new_mul(vec![
        Expr::new_int_lit("5"),
        Expr::new_int_lit("6"),
        Expr::new_int_lit("7"),
    ]);
    let s = "5 * 6 * 7";
    let mut scn = Scanner::new(s);
    assert_eq!(Expr::parse(&mut scn), Some(expr));
}

#[test]
fn parse_neg() {
    let expr = Expr::new_neg(Expr::new_grp(Expr::new_int_lit("75")));
    let s = "-(75)";
    let mut scn = Scanner::new(s);
    assert_eq!(Expr::parse(&mut scn), Some(expr));

    let expr = Expr::new_neg(Expr::new_grp(Expr::new_int_lit("-75")));
    let s = "-(-75)";
    let mut scn = Scanner::new(s);
    assert_eq!(Expr::parse(&mut scn), Some(expr));

    let expr = Expr::new_neg(Expr::new_grp(Expr::new_neg(Expr::new_grp(Expr::new_int_lit("75")))));
    let s = "-(-(75))";
    let mut scn = Scanner::new(s);
    assert_eq!(Expr::parse(&mut scn), Some(expr));
}

#[test]
fn parse_sub() {
    let expr = Expr::new_sub(vec![
        Expr::new_int_lit("7"),
        Expr::new_int_lit("8"),
        Expr::new_int_lit("9"),
    ]);
    let s = "7 - 8 - 9";
    let mut scn = Scanner::new(s);
    assert_eq!(Expr::parse(&mut scn), Some(expr));
}

#[test]
fn parse_int_lit() {
    let s = "7";
    let mut scn = Scanner::new(s);
    assert_eq!(Expr::parse(&mut scn), Some(Expr::new_int_lit("7")));

    let s = "-7";
    let mut scn = Scanner::new(s);
    assert_eq!(Expr::parse(&mut scn), Some(Expr::new_int_lit("-7")));
}

#[test]
fn parse_int_zero() {
    let s = "0";
    let mut scn = Scanner::new(s);
    assert_eq!(IntLit::parse(&mut scn), Some(IntLit::new("0")));
}

#[test]
fn parse_int_negative() {
    let s = "-1";
    let mut scn = Scanner::new(s);
    assert_eq!(IntLit::parse(&mut scn), Some(IntLit::new("-1")));
}

#[test]
fn parse_int_positive() {
    let s = "2";
    let mut scn = Scanner::new(s);
    assert_eq!(IntLit::parse(&mut scn), Some(IntLit::new("2")));
}

#[test]
fn parse_int_big() {
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

#[test]
fn parse_struct_anonymous_empty() {
    let s = "{}";
    let mut scn = Scanner::new(s);
    assert_eq!(StructLit::parse(&mut scn), Some(StructLit::default()));
}

#[test]
fn parse_struct_named_empty() {
    let s = "Monkey {}";
    let mut scn = Scanner::new(s);
    assert_eq!(StructLit::parse(&mut scn), Some(StructLit::new_named("Monkey", vec![])));
}

#[test]
fn parse_struct_property_value_shorthand() {
    let s = "Monkey { a , b , c }";
    let mut scn = Scanner::new(s);
    assert_eq!(StructLit::parse(&mut scn), Some(StructLit::new_named("Monkey", vec![
        StructLitArg::new_shorthand("a"),
        StructLitArg::new_shorthand("b"),
        StructLitArg::new_shorthand("c"),
    ])));

    let s = "BananaWithExtraComma { a , b , }";
    let mut scn = Scanner::new(s);
    assert_eq!(StructLit::parse(&mut scn), Some(StructLit::new_named("BananaWithExtraComma", vec![
        StructLitArg::new_shorthand("a"),
        StructLitArg::new_shorthand("b"),
    ])));
}

#[test]
fn parse_struct_with_property_values() {
    let s = "Monkey { a : 100 , b : 5 * 7 , c : true, d : {}, e : Vec { x, y, z } }";
    let mut scn = Scanner::new(s);
    assert_eq!(StructLit::parse(&mut scn), Some(StructLit::new_named("Monkey", vec![
        StructLitArg::new("a", Expr::new_int_lit("100")),
        StructLitArg::new("b", Expr::new_mul(vec![Expr::new_int_lit("5"), Expr::new_int_lit("7")])),
        StructLitArg::new("c", Expr::new_bool_lit(true)),
        StructLitArg::new("d", Expr::new_struct_anon_lit(vec![])),
        StructLitArg::new("e", Expr::new_struct_lit("Vec", vec![
            StructLitArg::new_shorthand("x"),
            StructLitArg::new_shorthand("y"),
            StructLitArg::new_shorthand("z"),
        ])),
    ])));

    let s = "BananaWithExtraComma { a : 100 , b : 5 * 7 , }";
    let mut scn = Scanner::new(s);
    assert_eq!(StructLit::parse(&mut scn), Some(StructLit::new_named("BananaWithExtraComma", vec![
        StructLitArg::new("a", Expr::new_int_lit("100")),
        StructLitArg::new("b", Expr::new_mul(vec![Expr::new_int_lit("5"), Expr::new_int_lit("7")])),
    ])));
}

#[test]
fn parse_var() {
    let s: &str = "monkey";
    let mut scn = Scanner::new(s);
    assert_eq!(Var::parse(&mut scn), Some(Var::new("monkey", Span::new(0, 0, 0, 6))));

    let s: &str = "monkey123";
    let mut scn = Scanner::new(s);
    assert_eq!(Var::parse(&mut scn), Some(Var::new("monkey123", Span::new(0, 0, 0, 9))));

    let s: &str = "123syntaxerror";
    let mut scn = Scanner::new(s);
    assert_eq!(Var::parse(&mut scn), None);
}
