#[derive(Debug, PartialEq)]
pub enum Expr {
    BoolLit(BoolLit),
    IntLit(IntLit),
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
pub struct Sub(Vec<Expr>);

impl Sub {
    pub fn new(ops: Vec<Expr>) -> Self {
        Self(ops)
    }
    
    pub fn ops(&self) -> &Vec<Expr> {
        &self.0
    }
}