use std::iter::Peekable;

use crate::tokenizer::Token;

#[derive(Debug)]
pub enum Expr {
	INT(i32),
	FUNC(Vec<Expr>),
}

fn parse_expr<T: Iterator<Item = Token>>(state: &mut Peekable<T>) -> Option<Expr> {
	match state.peek()? {
		Token::LPAR => {
			state.next()?;

			let mut exprs = vec![];
			while let Some(expr) = parse_expr(state) {
				exprs.push(expr);
			}

			// Should maybe make this return None here instead.
			assert!(!exprs.is_empty());
			
			(state.next()? == Token::RPAR)
				.then_some(Expr::FUNC(exprs))
		}
		Token::INT(nbr) => {
			// Not very elegant with the extra line as nbr can't be used after we do next
			let expr = Expr::INT(*nbr);
			state.next()?;
			Some(expr)
		}
		Token::RPAR => None
	}
}

pub fn parse(tokens: Vec<Token>) -> Option<Expr> {
   	parse_expr(&mut tokens.into_iter().peekable())
}