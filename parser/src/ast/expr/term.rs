use std::fmt::Debug;

use salmon_position::Locatable;

use crate::ast::expr::Expression;
use crate::error::FilePosition;

#[derive(Debug, Locatable)]
pub struct NumberTerm<'ctx> {
    pub position: FilePosition<'ctx>,
    pub number: &'ctx str,
}

impl<'ctx> Expression for NumberTerm<'ctx> {
    fn generate(&self) {}
}

#[derive(Debug, Locatable)]
pub struct ParamTerm<'ctx> {
    pub position: FilePosition<'ctx>,
    pub name: String,
    pub pure: bool,
}

impl<'ctx> Expression for ParamTerm<'ctx> {
    fn generate(&self) {}
}

#[derive(Debug, Locatable)]
pub struct Term<'ctx> {
    pub position: FilePosition<'ctx>,
    pub values: Vec<Box<dyn Expression + 'ctx>>,
}

impl<'ctx> Expression for Term<'ctx> {
    fn generate(&self) {}
}
