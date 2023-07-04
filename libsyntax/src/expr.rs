use std::vec;

use libast::expr::{BoolLit, Expr, IntLit, StructLit, Var, StructLitArg};
use crate::{common::{parse_bool, parse_id, parse_int, try_parse}, scanner::{Parse, Scanner}};

impl Parse for Expr {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<Self> {
        parse_sub(scn)
    }
}

impl Parse for BoolLit {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<Self> {
        parse_bool(scn).map(BoolLit::new)
    }
}

impl Parse for IntLit {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<Self> {
        parse_int(scn).map(IntLit::new)
    }
}

impl Parse for StructLit {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<Self> {
        let id = parse_id(scn);
        scn.skip_spaces();
        scn.scan("{")?;
        scn.skip_whitespaces();
        
        if scn.has("}") {
            return Some(StructLit::new(id, vec![]));
        }

        let mut args = vec![];

        loop {
            let arg = StructLitArg::parse(scn)?;
            args.push(arg);

            scn.skip_whitespaces();
            
            if scn.has(",") {
                scn.skip_whitespaces();

                if scn.has("}") {
                    break;    
                }

                continue;
            }           
            else if scn.has("}") {
                break;    
            }
        }

        Some(StructLit::new(id, args))
    }
}

impl Parse for StructLitArg {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<Self> {
        let id = parse_id(scn)?;
        scn.skip_spaces();
        let mut expr = None;
        
        if scn.has(":") {
            scn.skip_spaces();
            expr = Expr::parse(scn);
        }

        Some(StructLitArg::new(id, expr))
    }
}

impl Parse for Var {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<Var> {
        let cur0 = scn.current_cursor();
        let id = parse_id(scn)?;
        let cur1 = scn.current_cursor();
        Some(Var::new(id, cur0.span_to(cur1)))
    }
}

fn parse_add(scn: &mut Scanner) -> Option<Expr> {
    let mut ops = vec![parse_div(scn)?];
    
    loop {
        let mut try_scn = scn.clone();
        try_scn.skip_spaces();

        if try_scn.has("+") {
            scn.replace(try_scn); 
            scn.skip_spaces();
            ops.push(parse_div(scn)?);
        }
        else {
            break;
        }
    }

    if ops.len() == 1 {
        return Some(ops.remove(0));
    }

    Some(Expr::new_add(ops))
}

fn parse_base(scn: &mut Scanner) -> Option<Expr> {
    try_parse(scn, |s| BoolLit::parse(s).map(Expr::BoolLit))
    .or_else(|| try_parse(scn, |s| IntLit::parse(s).map(Expr::IntLit)))
    .or_else(|| try_parse(scn, |s| StructLit::parse(s).map(Expr::StructLit)))
    .or_else(|| try_parse(scn, |s| Var::parse(s).map(Expr::Var)))
    .or_else(|| try_parse(scn, parse_maybe_neg))
}

fn parse_div(scn: &mut Scanner) -> Option<Expr> {
    let mut ops = vec![parse_mul(scn)?];
    
    loop {
        let mut try_scn = scn.clone();
        try_scn.skip_spaces();

        if try_scn.has("/") {
            scn.replace(try_scn); 
            scn.skip_spaces();
            ops.push(parse_mul(scn)?);
        }
        else {
            break;
        }
    }

    if ops.len() == 1 {
        return Some(ops.remove(0));
    }

    Some(Expr::new_div(ops))
}

fn parse_grp(scn: &mut Scanner) -> Option<Expr> {
    if !scn.has("(") {
        return None;
    }

    let expr = Expr::parse(scn)?;

    if !scn.has(")") {
        return None;
    }

    Some(Expr::new_grp(expr))
}

fn parse_maybe_neg(scn: &mut Scanner) -> Option<Expr> {
    let negated = scn.has("-");
    let grp = parse_grp(scn)?;

    if negated {
        return Some(Expr::new_neg(grp));
    }

    Some(grp)
}

fn parse_mul(scn: &mut Scanner) -> Option<Expr> {
    let mut ops = vec![parse_base(scn)?];
    
    loop {
        let mut try_scn = scn.clone();
        try_scn.skip_spaces();

        if try_scn.has("*") {
            scn.replace(try_scn); 
            scn.skip_spaces();
            ops.push(parse_base(scn)?);
        }
        else {
            break;
        }
    }

    if ops.len() == 1 {
        return Some(ops.remove(0));
    }

    Some(Expr::new_mul(ops))
}

fn parse_sub(scn: &mut Scanner) -> Option<Expr> {
    let mut ops = vec![parse_add(scn)?];
    
    loop {
        let mut try_scn = scn.clone();
        try_scn.skip_spaces();

        if try_scn.has("-") {
            scn.replace(try_scn); 
            scn.skip_spaces();
            ops.push(parse_add(scn)?);
        }
        else {
            break;
        }
    }

    if ops.len() == 1 {
        return Some(ops.remove(0));
    }

    Some(Expr::new_sub(ops))
}