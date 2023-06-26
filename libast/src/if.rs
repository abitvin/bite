use crate::block::Block;

#[derive(Debug, PartialEq)]
pub struct If {
    if_block: (bool, Block),
    elif_blocks: Vec<(bool, Block)>,
    else_block: Option<Block> 
}

impl If {
    pub fn new(if_block: (bool, impl Into<Block>), elif_blocks: Vec<(bool, Block)>, else_block: impl Into<Option<Block>>) -> Self {
        Self { 
            if_block: (if_block.0, if_block.1.into()),
            elif_blocks,
            else_block: else_block.into(),
        }
    }
}