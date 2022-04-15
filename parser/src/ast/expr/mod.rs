use std::fmt::Debug;

use salmon_position::Locatable;

use crate::error::FilePosition;

///
/// Represents an expression.
///
/// All expressions must inherit this trait.
///
pub trait Expression: Debug + crate::ast::utils::Locatable {
    fn generate(&self);
}

///
/// Represents binary operations, such as `+`, `*`.
///
/// Please note that
///
#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

///
/// Represents
///
#[derive(Debug, Locatable)]
pub struct BinaryOpExpr<'ctx> {
    pub position: FilePosition<'ctx>,
    pub op: BinaryOp,
    pub left: Box<dyn Expression + 'ctx>,
    pub right: Box<dyn Expression + 'ctx>,
}

impl<'ctx> Expression for BinaryOpExpr<'ctx> {
    fn generate(&self) {}
}

pub mod term;
