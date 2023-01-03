use std::collections::HashMap;

use crate::parser::Expr;

pub struct Namespace<'a> {
	symbols: HashMap<i64, Expr>,
	base: Option<&'a Namespace<'a>>, 
}

pub fn interpret(expr: Expr) {
    
}