use libsyntax::{scanner::Scanner, r#if::If, stmt::Stmt, var_decl::VarDecl, block::Block};

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
    let iff = If::new((true, vec![]), vec![(true, Block::new())], None);
    assert_eq!(If::parse(&mut scn), Some(iff))
}

#[test]
fn parse_populated_elif_block() {
    unimplemented!()
    // let s = "if true
    //                elif true
    //                     a = 567
    //                .";
    // let mut scn = Scanner::new(s);
    // let elif_blocks = vec![
    //     (true, Block::new())
    // ];
    // let iff = If::new((true, vec![]), elif_blocks, None);
    // assert_eq!(If::parse(&mut scn), Some(iff))
}

#[test]
fn parse_empty_else_block() {
    unimplemented!()
    // let s = "if true
    //                elif true
    //                else
    //                .";
    // let mut scn = Scanner::new(s);
    // let iff = If::new((true, vec![]), vec![], None);
    // assert_eq!(If::parse(&mut scn), Some(iff))
}

#[test]
fn parse_populated_else_block() {
    unimplemented!()
    // let s = "if true
    //                elif true
    //                else
    //                     cheese = 100
    //                .";
    // let mut scn = Scanner::new(s);
    // let iff = If::new((true, vec![]), vec![], None);
    // assert_eq!(If::parse(&mut scn), Some(iff))
}
