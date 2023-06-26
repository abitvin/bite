use crate::{block::Block, param::Param};

#[derive(Debug, PartialEq)]
pub struct FnDecl {
    id: String,
    params: Vec<Param>,
    ret_type: String,
    block: Block,
}

impl FnDecl {
    pub fn new(id: impl Into<String>, params: Vec<Param>, ret_type: impl Into<String>, block: impl Into<Block>) -> Self {
        Self { 
            id: id.into(), 
            params, ret_type: 
            ret_type.into(), 
            block: block.into(),
        }
    }
}