extern crate grammar;
use grammar::{CompiledGrammar, Grammar};
use super::param::Param;

pub struct FnDecl {
    id: String,
    params: Vec<Param>,
}

impl FnDecl {
    fn from_id(value: &str) -> Self {
        Self { id: String::from(value), params: vec![] }
    }

    fn from_params(value: Vec<Param>) -> Self {
        Self { id: String::new(), params: value }
    }
}

pub struct FnDeclParser(CompiledGrammar<FnDecl>);

impl FnDeclParser {
    fn new() -> Self {
        Self(new_fn_decl_parser())
    }
}

fn new_fn_decl_parser() -> CompiledGrammar<FnDecl> {
    let mut g = Grammar::new();
    g.map("id", "[a-zA-Z][0-9a-zA-Z]", |_, s| Ok(FnDecl::from_id(s)));
    
    // TODO We need this.
    // g.map_from(other, "params", "<param>( , <param>)")
    g.compile().unwrap()
}