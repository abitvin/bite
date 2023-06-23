use libsyntax::{block::Block, fn_decl::FnDecl, param::Param, scanner::Scanner, stmt::Stmt, var_decl::VarDecl};

#[test]
fn parse_fn() {
    let code = "main : () -> void
                      .";

    let mut scn = Scanner::new(code);
    assert_eq!(FnDecl::parse(&mut scn), Some(FnDecl::new("main", vec![], "void", Block::new())));
}

#[test]
fn parse_fn_with_params() {
    let code = "add : ( a : i32 , b : i32 ) -> i32
                      .";

    let params = vec![Param::new("a", "i32"), Param::new("b", "i32")];
    
    let mut scn = Scanner::new(code);
    assert_eq!(FnDecl::parse(&mut scn), Some(FnDecl::new("add", params, "i32", Block::new())));
}

#[test]
fn parse_fn_with_var_decls() {
    let code = "eat : ( cheese : Cheese ) -> void
                        c = 123
                        d = 456
                      .";

    let params = vec![Param::new("cheese", "Cheese")];
    let stmts = vec![
        Stmt::VarDecl(VarDecl::new("c", None, "123")),
        Stmt::VarDecl(VarDecl::new("d", None, "456"))
    ];
    
    let mut scn = Scanner::new(code);
    assert_eq!(FnDecl::parse(&mut scn), Some(FnDecl::new("eat", params, "void", stmts)));
}