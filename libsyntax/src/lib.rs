extern crate grammar;
use grammar::{CompiledGrammar, Grammar, GrammarError};

pub struct Parser{
    parser_grouped_7: CompiledGrammar<BoxedSeven>
}

impl Parser {
    pub fn new() -> Self {
        let parser_7 = parser_7();
        let parser_grouped_7 = parser_grouped_7(parser_7);
        
        Self { parser_grouped_7 }
    }

    pub fn scan(&self, expr: &str) -> Result<BoxedSeven, GrammarError> {
        self.parser_grouped_7.scan("grouped", expr)
            .map(|mut x| x.swap_remove(0))
    }
}

pub struct BoxedSeven(i32);

fn parser_7() -> CompiledGrammar<i32> {
    let mut g = Grammar::new();
    g.map("seven", "7", |_, _| Ok(7));
    g.compile().unwrap()
}

fn parser_grouped_7(parser_7: CompiledGrammar<i32>) -> CompiledGrammar<BoxedSeven> {
    let mut g = Grammar::new();
    g.map("seven", "7", move |_, s| {
        parser_7
            .scan("seven", s)
            .map(|x| BoxedSeven(x[0]))
            .map_err(|x| x.to_string())
    });
    g.rule("grouped", "\\(<seven>\\)");
    g.compile().unwrap()
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn mul(left: usize, right: usize) -> usize {
    left * right
}

pub fn sub(left: usize, right: usize) -> usize {
    left - right
}