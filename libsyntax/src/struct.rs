extern crate grammar;
use grammar::{CompiledGrammar, Grammar, GrammarError};
use super::param::Param;

struct Struct {
    id: String,
    props: Vec<Param>,
}

