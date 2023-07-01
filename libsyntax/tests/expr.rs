use libast::expr::Expr;
use libsyntax::scanner::{Parse, Scanner};

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