use libsyntax::{r#fn::FnDecl, param::Param, scanner::Scanner};

#[test]
fn parse_fn() {
    let params = vec![Param::new("a", "i32"), Param::new("b", "i32")];
    let d = FnDecl::new("add", params, "i32");
    let code = "add : ( a : i32 , b : i32 ) -> i32 \n. \n";
    let mut scn = Scanner::new(code);
    
    assert_eq!(FnDecl::parse(&mut scn), Some(d));
}