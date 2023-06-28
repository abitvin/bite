use libast::{block::Block, fn_decl::FnDecl, param::Param, stmt::Stmt};
use libsyntax::scanner::{Parse, Scanner};

#[test]
fn parse_fn() {
    let code = "main : () -> void
                      .";

    let mut scn = Scanner::new(code);
    assert_eq!(FnDecl::parse(&mut scn), Some(FnDecl::new(None, "main", vec![], "void", Block::default())));
}

#[test]
fn parse_fn_witn_namespace() {
    let code = "Cheese.new : () -> Cheese
                      .";

    let mut scn = Scanner::new(code);
    assert_eq!(FnDecl::parse(&mut scn), Some(FnDecl::new(Some(String::from("Cheese")), "new", vec![], "Cheese", Block::default())));
}

#[test]
fn parse_fn_with_params() {
    let code = "add : ( a : i32 , b : i32 ) -> i32
                      .";

    let params = vec![Param::new("a", "i32"), Param::new("b", "i32")];
    
    let mut scn = Scanner::new(code);
    assert_eq!(FnDecl::parse(&mut scn), Some(FnDecl::new(None, "add", params, "i32", Block::default())));
}

#[test]
fn parse_fn_with_var_decls() {
    let code = "eat : ( cheese : Cheese ) -> void
                        c = 123
                        d = 456
                      .";

    let params = vec![Param::new("cheese", "Cheese")];
    let stmts = vec![
        Stmt::new_var_decl("c", None, "123"),
        Stmt::new_var_decl("d", None, "456")
    ];
    
    let mut scn = Scanner::new(code);
    assert_eq!(FnDecl::parse(&mut scn), Some(FnDecl::new(None, "eat", params, "void", stmts)));
}