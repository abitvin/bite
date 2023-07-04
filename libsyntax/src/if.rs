use crate::{common::{parse_bool, parse_id, try_parse}, scanner::{Parse, Scanner}};
use libast::{block::Block, r#if::{If, IfExpr, IfOperand, IfOperator}};

impl Parse for If {
    type Item = Self;
    
    fn parse(scn: &mut Scanner) -> Option<Self> {
        if !scn.has("if") {
            return None;
        }

        scn.skip_spaces();     
        let expr = IfExpr::parse(scn)?;
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
                let expr = IfExpr::parse(scn)?;
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

impl Parse for IfExpr {
    type Item = Self;
    
    fn parse(scn: &mut Scanner) -> Option<Self> {
        let left = IfOperand::parse(scn)?;
        scn.skip_whitespaces();

        match IfOperator::parse(scn) {
            Some(op) => {
                scn.skip_whitespaces();
                let right = IfOperand::parse(scn)?;
                Some(IfExpr::Cmp((left, op, right)))
            },
            _ => {
                Some(IfExpr::Operand(left))
            },
        }
    }
}

impl Parse for IfOperand {
    type Item = Self;
    
    fn parse(scn: &mut Scanner) -> Option<Self> {
        try_parse(scn, |s| parse_bool(s).map(IfOperand::BoolLit))
        .or_else(|| try_parse(scn, |s| parse_id(s).map(IfOperand::Var)))
    }
}

impl Parse for IfOperator {
    type Item = Self;
    
    fn parse(scn: &mut Scanner) -> Option<Self> {
        if scn.has("==") {
            Some(IfOperator::Eq)
        }
        else if scn.has("!=") {
            Some(IfOperator::Neq)
        }
        else if scn.has(">=") {
            Some(IfOperator::Ge)
        }
        else if scn.has(">") {
            Some(IfOperator::Gt)
        }
        else if scn.has("<=") {
            Some(IfOperator::Le)
        }
        else if scn.has("<") {
            Some(IfOperator::Lt)
        }
        else {
            None
        }
    }
}