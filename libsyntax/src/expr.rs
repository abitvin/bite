use libast::{bool_lit::BoolLit, expr::Expr, int_lit::IntLit};
use crate::{common::try_parse, scanner::{Parse, Scanner}};

impl Parse for Expr {
    type Item = Self;

    fn parse(scn: &mut Scanner) -> Option<Self> {
        parse_sub(scn)
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
    try_parse(scn, |s| {
        BoolLit::parse(s).map(Expr::BoolLit)
    })
    .or_else(|| try_parse(scn, |s| {
        IntLit::parse(s).map(Expr::IntLit)
    }))
    .or_else(|| try_parse(scn, |s| {
        parse_maybe_neg(s)
    }))
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