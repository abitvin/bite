extern crate grammar;
use grammar::{CompiledGrammar, Grammar};

pub struct Param {
    pub id: String,
    pub typ: String,
}

impl Param {
    fn from_id(value: &str) -> Self {
        Self { id: String::from(value), typ: String::new() }
    }

    fn from_type(value: &str) -> Self {
        Self { id: String::new(), typ: String::from(value) }
    }

    fn from_vec(mut params: Vec<Param>) -> Self {
        let id = params.swap_remove(0).id;
        let typ = params.swap_remove(0).typ;
        Self {id, typ}
    }
}

pub struct ParamParser(CompiledGrammar<Param>);

impl ParamParser {
    pub fn new() -> Self {
        Self(new_param_parser())
    }

    pub fn scan(&self, expr: &str) -> Result<Param, grammar::GrammarError> {
        self.0.scan("param", expr)
            .map(|mut p| p.swap_remove(0))
    }
}

fn new_param_parser() -> CompiledGrammar<Param> {
    let mut g = Grammar::new();
    g.map("id", "[a-zA-Z][0-9a-zA-Z]*", |_, s| Ok(Param::from_id(s)));  // TODO This is weird.
    g.map("type", "[a-zA-Z][0-9a-zA-Z]*", |_, s| Ok(Param::from_type(s))); // ..
    g.map("param", "<id> : <type>", |p, _| Ok(Param::from_vec(p))); // ..
    g.compile().unwrap()
}