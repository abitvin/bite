use crate::{block::Block, param::Param};

#[derive(Debug, PartialEq)]
pub struct FnDecl {
    ns: Option<String>,
    id: String,
    params: Vec<Param>,
    ret_type: String,
    block: Block,
}

impl FnDecl {
    pub fn new(ns: impl Into<Option<String>>, id: impl Into<String>, params: Vec<Param>, ret_type: impl Into<String>, block: impl Into<Block>) -> Self {
        Self {
            ns: ns.into(), 
            id: id.into(), 
            params, ret_type: 
            ret_type.into(), 
            block: block.into(),
        }
    }
}