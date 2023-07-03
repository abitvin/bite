use libast::{block::Block, expr::Expr, r#if::If, stmt::Stmt};
use libsyntax::scanner::{Parse, Scanner};

#[test]
fn parse_empty_if_block() {
    let s = "if true
                   .";
    let mut scn = Scanner::new(s);
    let iff = If::new((true, vec![]), vec![], None);
    assert_eq!(If::parse(&mut scn), Some(iff))
}

#[test]
fn parse_populated_if_block() {
    let s = "if false
                        x = 1
                        y = 3
                   .";
    let mut scn = Scanner::new(s);
    
    let if_block = vec![
        Stmt::new_var_decl("x", None, Expr::new_int_lit("1")),
        Stmt::new_var_decl("y", None, Expr::new_int_lit("3"))
    ];
    let iff = If::new((false, if_block), vec![], None);

    assert_eq!(If::parse(&mut scn), Some(iff))
}

#[test]
fn parse_empty_elif_block() {
    let s = "if true
                   elif true
                   .";
    let mut scn = Scanner::new(s);
    let iff = If::new((true, vec![]), vec![(true, Block::default())], None);
    assert_eq!(If::parse(&mut scn), Some(iff))
}

#[test]
fn parse_populated_elif_block() {
    let s = "if true
                   elif true
                        a = 567
                   .";
    let mut scn = Scanner::new(s);
    let elif_blocks = vec![
        (true, Block::new(vec![Stmt::new_var_decl("a", None, Expr::new_int_lit("567"))]))
    ];
    let iff = If::new((true, vec![]), elif_blocks, None);
    assert_eq!(If::parse(&mut scn), Some(iff))
}

#[test]
fn parse_empty_else_block() {
    let s = "if true
                   else
                   .";
    let mut scn = Scanner::new(s);
    let else_block = Block::new(vec![]);
    let iff = If::new((true, vec![]), vec![], Some(else_block));
    assert_eq!(If::parse(&mut scn), Some(iff))
}

#[test]
fn parse_populated_else_block() {
    let s = "if true
                   else
                        cheese = 100
                   .";
    let mut scn = Scanner::new(s);
    let else_block = Block::new(vec![Stmt::new_var_decl("cheese", None, Expr::new_int_lit("100"))]);
    let iff = If::new((true, vec![]), vec![], Some(else_block));
    assert_eq!(If::parse(&mut scn), Some(iff))
}
