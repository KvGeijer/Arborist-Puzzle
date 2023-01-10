use crate::parser::Expr;

use rand::Rng;

const REPL_PROB: u8 = 8;

struct State {
	prog: String,
	last_int: bool,
}

pub fn generate(expr: &Expr) -> String {
	let mut state = State{
		prog: String::new(), 
		last_int: false
	};
	let mut rng = rand::thread_rng();

	state.gen(expr, &mut rng);

	state.prog
}

impl State {
	fn gen<T: Rng>(&mut self, expr: &Expr, rng: &mut T) {
		match expr {
			Expr::INT(nbr) => {
				if rng.gen_range(0..REPL_PROB) == 0 {
					let sub = rng.gen_range(-20..=20);
					let temp_expr = Expr::FUNC(vec![
						Expr::INT(3),
						Expr::INT(*nbr + sub),
						Expr::INT(sub),
					]);
					self.gen(&temp_expr, rng);
				} else {
					if self.last_int {
						self.prog.push(' ');
					}
					self.prog.push_str(&nbr.to_string());
					self.last_int = true;
				}
			}
			Expr::FUNC(exprs) => {
				self.prog.push('(');
				self.last_int = false;

				for expr in exprs {
					self.gen(expr, rng);
				}
				
				self.prog.push(')');
				self.last_int = false;
			}
		}
	}
}