use libsyntax::{scanner::{Parse, Scanner}, r#if::If, stmt::Stmt, var_decl::VarDecl, block::Block};

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
        Stmt::VarDecl(VarDecl::new("x", None, "1")),
        Stmt::VarDecl(VarDecl::new("y", None, "3"))
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
        (true, Block::new(vec![Stmt::VarDecl(VarDecl::new("a", None, "567"))]))
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
    let else_block = Block::new(vec![Stmt::VarDecl(VarDecl::new("cheese", None, "100"))]);
    let iff = If::new((true, vec![]), vec![], Some(else_block));
    assert_eq!(If::parse(&mut scn), Some(iff))
}
