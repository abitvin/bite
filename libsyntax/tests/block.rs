use std::vec;

use libast::{block::Block, stmt::Stmt};
use libsyntax::scanner::{Parse, Scanner};

#[test]
fn parse_block() {
    let s = "i = 10
                if true
                .
                j = 20
                k = 30
                if true
                elif false
                .
                if false
                else
                .";

    let block = Block::new(vec![
        Stmt::new_var_decl("i", None, "10"),
        Stmt::new_if((true, Block::default()), vec![], None),
        Stmt::new_var_decl("j", None, "20"),
        Stmt::new_var_decl("k", None, "30"),
        Stmt::new_if((true, Block::default()), vec![(false, Block::default())], None),
        Stmt::new_if((false, Block::default()), vec![], Block::default()),
    ]);

    let mut scn = Scanner::new(s);
    assert_eq!(Block::parse(&mut scn), Some(block));
}