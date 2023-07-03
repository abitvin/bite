use crate::block::Block;

#[derive(Debug, PartialEq)]
pub struct If {
    if_block: (IfExpr, Block),
    elif_blocks: Vec<(IfExpr, Block)>,
    else_block: Option<Block> 
}

impl If {
    pub fn new(if_block: (IfExpr, impl Into<Block>), elif_blocks: Vec<(IfExpr, Block)>, else_block: impl Into<Option<Block>>) -> Self {
        Self { 
            if_block: (if_block.0, if_block.1.into()),
            elif_blocks,
            else_block: else_block.into(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum IfExpr {
    Cmp((IfOperand, IfOperator, IfOperand)),
    Operand(IfOperand),
}

impl IfExpr {
    pub fn new_bool(value: bool) -> Self {
        Self::Operand(IfOperand::BoolLit(value))
    }

    pub fn new_cmp_eq_vars(left: impl Into<String>, right: impl Into<String>) -> Self {
        Self::Cmp((IfOperand::Var(left.into()), IfOperator::Eq, IfOperand::Var(right.into())))
    }

    pub fn new_cmp_neq_vars(left: impl Into<String>, right: impl Into<String>) -> Self {
        Self::Cmp((IfOperand::Var(left.into()), IfOperator::Neq, IfOperand::Var(right.into())))
    }

    pub fn new_cmp_le_vars(left: impl Into<String>, right: impl Into<String>) -> Self {
        Self::Cmp((IfOperand::Var(left.into()), IfOperator::Le, IfOperand::Var(right.into())))
    }

    pub fn new_cmp_lt_vars(left: impl Into<String>, right: impl Into<String>) -> Self {
        Self::Cmp((IfOperand::Var(left.into()), IfOperator::Lt, IfOperand::Var(right.into())))
    }

    pub fn new_cmp_ge_vars(left: impl Into<String>, right: impl Into<String>) -> Self {
        Self::Cmp((IfOperand::Var(left.into()), IfOperator::Ge, IfOperand::Var(right.into())))
    }

    pub fn new_cmp_gt_vars(left: impl Into<String>, right: impl Into<String>) -> Self {
        Self::Cmp((IfOperand::Var(left.into()), IfOperator::Gt, IfOperand::Var(right.into())))
    }

    pub fn new_var(id: impl Into<String>) -> Self {
        Self::Operand(IfOperand::Var(id.into()))
    }
}

#[derive(Debug, PartialEq)]
pub enum IfOperand {
    BoolLit(bool),
    Var(String),
}

#[derive(Debug, PartialEq)]
pub enum IfOperator {
    Eq,
    Neq,
    Ge,
    Gt,
    Le,
    Lt,
}