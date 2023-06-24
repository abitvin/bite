use crate::{block::Block, common::parse_bool, scanner::Scanner};

#[derive(Debug, PartialEq)]
pub struct If {
    if_block: (bool, Block),
    elif_blocks: Vec<(bool, Block)>,
    else_block: Option<Block> 
}

impl If {
    pub fn new(if_block: (bool, impl Into<Block>), elif_blocks: Vec<(bool, Block)>, else_block: Option<Block>) -> Self {
        Self { 
            if_block: (if_block.0, if_block.1.into()),
            elif_blocks,
            else_block,
        }
    }

    pub fn parse(scn: &mut Scanner) -> Option<Self> {
        if !scn.has("if") {
            return None;
        }

        scn.skip_spaces();     
        let expr = parse_bool(scn)?;
        scn.skip_spaces();
        scn.skip_newline();
        let if_block = (expr, Block::parse(scn)?);

        let mut elif_blocks = vec![];
        
        loop {
            scn.skip_spaces();
            let mut try_scn = scn.clone();
            
            if try_scn.has("elif") {
                scn.replace(try_scn);
                scn.skip_spaces();
                let expr = parse_bool(scn)?;
                scn.skip_spaces();
                scn.skip_newline();
                
                elif_blocks.push((expr, Block::parse(scn)?));
            }
            else {
                break;
            }
        }

        let mut else_block = None;
        scn.skip_spaces();
        let mut try_scn = scn.clone();
            
        if try_scn.has("else") {
            scn.replace(try_scn);
            scn.skip_spaces();
            scn.skip_newline();
            
            else_block = Some(Block::parse(scn)?);
        }

        scn.skip_spaces();
        scn.scan(".")?;
        
        Some(Self { if_block, elif_blocks, else_block })
    }
}