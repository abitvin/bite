extern crate grammar;
use grammar::{CompiledGrammar, Grammar};

pub struct BoxedSeven(i32);

fn parser_7() -> CompiledGrammar<i32> {
    let mut g = Grammar::new();
    g.map("seven", "7", |_, _| Ok(7));
    g.compile().unwrap()
}

pub fn parser_grouped_7() -> CompiledGrammar<BoxedSeven> {
    let mut g = Grammar::new();
    g.map("seven", "7", |_, s| {
        parser_7()
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