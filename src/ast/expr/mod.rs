use std::fmt::Debug;

use crate::error::FilePosition;

pub trait Expression: Debug {
    fn generate(&self);
}

#[derive(Debug)]
pub struct AddExpr<'ctx> {
    pub position: FilePosition<'ctx>,
    pub is_add: bool,
    pub left: Box<dyn Expression + 'ctx>,
    pub right: Box<dyn Expression + 'ctx>,
}

impl<'ctx> Expression for AddExpr<'ctx> {
    fn generate(&self) {}
}

pub mod term;
