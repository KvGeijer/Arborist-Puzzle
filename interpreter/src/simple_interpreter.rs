use crate::parser::Expr;

pub fn interpret(expr: &Expr) -> i32 {
    eval(expr)
}

fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::INT(nbr) => *nbr,
        Expr::FUNC(exprs) => {
            let op = eval(&exprs[0]);
            match op {
                1 => {
                    // If
                    let cond = eval(&exprs[1]);
                    let ind = 2 + (cond <= 0) as usize;
                    eval(&exprs[ind])
                }
                3 => {
                    // Sub
                    let a = eval(&exprs[1]);
                    let b = eval(&exprs[2]);
                    a - b
                }
                _nbr => {
                    // Sum
					exprs.into_iter()
						.map(|expr| eval(expr))
						.sum()
                }
            }
        }
    }
}
