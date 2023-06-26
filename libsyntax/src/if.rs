use crate::{common::parse_bool, scanner::{Parse, Scanner}};
use libast::{block::Block, r#if::If};

impl Parse for If {
    type Item = Self;
    
    fn parse(scn: &mut Scanner) -> Option<Self> {
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
        
        Some(Self::new(if_block, elif_blocks, else_block))
    }
}