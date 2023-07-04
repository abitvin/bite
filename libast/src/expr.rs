use crate::span::Span;

#[derive(Debug, PartialEq)]
pub enum Expr {
    BoolLit(BoolLit),
    IntLit(IntLit),
    StructLit(StructLit),
    Var(Var),
    Add(Add),
    Div(Div),
    Group(Group),
    Mul(Mul),
    Neg(Neg),
    Sub(Sub),
}

impl Expr {
    pub fn new_add(ops: Vec<Expr>) -> Expr {
        Expr::Add(Add::new(ops))
    }

    pub fn new_div(ops: Vec<Expr>) -> Expr {
        Expr::Div(Div::new(ops))
    }

    pub fn new_grp(expr: impl Into<Box<Expr>>) -> Expr {
        Expr::Group(Group::new(expr))
    }

    pub fn new_mul(ops: Vec<Expr>) -> Expr {
        Expr::Mul(Mul::new(ops))
    }

    pub fn new_neg(expr: impl Into<Box<Expr>>) -> Expr {
        Expr::Neg(Neg::new(expr))
    }

    pub fn new_sub(ops: Vec<Expr>) -> Expr {
        Expr::Sub(Sub::new(ops))
    }

    pub fn new_bool_lit(val: bool) -> Expr {
        Expr::BoolLit(BoolLit::new(val))
    }

    pub fn new_int_lit(val: impl Into<String>) -> Expr {
        Expr::IntLit(IntLit::new(val.into()))
    }

    pub fn new_struct_lit(id: impl Into<String>, args: Vec<StructLitArg>) -> Expr {
        Expr::StructLit(StructLit::new_named(id, args))
    }

    pub fn new_struct_anon_lit(args: Vec<StructLitArg>) -> Expr {
        Expr::StructLit(StructLit::new_anon(args))
    }

    pub fn new_var(id: impl Into<String>, span: Span) -> Expr {
        Expr::Var(Var::new(id, span))
    }
}

#[derive(Debug, PartialEq)]
pub struct Add(Vec<Expr>);

impl Add {
    pub fn new(ops: Vec<Expr>) -> Self {
        Self(ops)
    }

    pub fn ops(&self) -> &Vec<Expr> {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct BoolLit(bool);

impl BoolLit {
    pub fn new(val: bool) -> Self {
        Self(val)
    }
}

#[derive(Debug, PartialEq)]
pub struct Div(Vec<Expr>);

impl Div {
    pub fn new(ops: Vec<Expr>) -> Self {
        Self(ops)
    }
    
    pub fn ops(&self) -> &Vec<Expr> {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct Group(Box<Expr>);

impl Group {
    pub fn new(expr: impl Into<Box<Expr>>) -> Self {
        Self(expr.into())
    }
}

#[derive(Debug, PartialEq)]
pub struct IntLit(String);

impl IntLit {
    pub fn new(val: impl Into<String>) -> Self {
        Self(val.into())
    }
}

#[derive(Debug, PartialEq)]
pub struct Mul(Vec<Expr>);

impl Mul {
    pub fn new(ops: Vec<Expr>) -> Self {
        Self(ops)
    }
    
    pub fn ops(&self) -> &Vec<Expr> {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct Neg(Box<Expr>);

impl Neg {
    pub fn new(expr: impl Into<Box<Expr>>) -> Self {
        Self(expr.into())
    }
}

#[derive(Debug, PartialEq)]
pub struct StructLit {
    id: Option<String>,
    args: Vec<StructLitArg>,
}

impl StructLit {
    pub fn new(id: Option<String>, args: Vec<StructLitArg>) -> Self {
        Self { id, args }
    }

    pub fn new_named(id: impl Into<String>, args: Vec<StructLitArg>) -> Self {
        Self { id: Some(id.into()), args }
    }

    pub fn new_anon(args: Vec<StructLitArg>) -> Self {
        Self { id: None, args }
    }
}

impl Default for StructLit {
    fn default() -> Self {
        Self { id: None, args: vec![] }
    }
}

#[derive(Debug, PartialEq)]
pub struct StructLitArg {
    id: String,
    expr: Option<Expr>,
}

impl StructLitArg {
    pub fn new(id: impl Into<String>, expr: impl Into<Option<Expr>>) -> Self {
        Self { id: id.into(), expr: expr.into() }
    }

    pub fn new_shorthand(id: impl Into<String>) -> Self {
        Self { id: id.into(), expr: None }
    }
}

#[derive(Debug, PartialEq)]
pub struct Sub(Vec<Expr>);

impl Sub {
    pub fn new(ops: Vec<Expr>) -> Self {
        Self(ops)
    }
    
    pub fn ops(&self) -> &Vec<Expr> {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct Var {
    id: String,
    span: Span,
}

impl Var {
    pub fn new(id: impl Into<String>, span: Span) -> Self {
        Self { id: id.into(), span }
    }
}