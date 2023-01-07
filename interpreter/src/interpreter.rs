use typed_arena::Arena;

use crate::parser::Expr;

struct Namespace<'a> {
	binding: (i32, &'a Expr),
	base: Option<&'a Namespace<'a>>,
}

pub fn interpret(expr: &Expr) -> i32 {
    // Could get rid of this, but fun to try
    let arena = Arena::new();
    eval(expr, None, &arena)
}

fn eval<'a>(expr: &'a Expr, base: Option<&'a Namespace<'a>>, arena: &'a Arena<Namespace<'a>>) -> i32 {
    match expr {
        Expr::INT(nbr) => *nbr,
        Expr::FUNC(exprs) => {
            let op = eval(&exprs[0], base, arena);
            match op {
                0 => {
                    // Bind
                    let id = eval(&exprs[1], base, arena);
                    let ns = arena.alloc(Namespace{binding: (id, &exprs[2]), base: base});
                    eval(&exprs[3], Some(ns), arena)
                }
                1 => {
                    // If
                    let cond = eval(&exprs[1], base, arena);
                    let ind = 2 + (cond > 0) as usize;
                    eval(&exprs[ind], base, arena)
                }
                2 => {
                    // Print
                    let nbr = eval(&exprs[1], base, arena);
                    print!("{}", nbr as u8 as char);
                    nbr
                }
                3 => {
                    // Sub
                    let a = eval(&exprs[1], base, arena);
                    let b = eval(&exprs[2], base, arena);
                    a - b
                }
                nbr => {
                    // Normal function
                    // into_iter?
                    let args: Vec<i32> = exprs[1..]
                        .into_iter()
                        .map(|expr| eval(expr, base, arena))
                        .collect();
                    lookup(nbr, args, base, arena)
                }
            }
        }
    }
}

fn lookup<'a>(id: i32, args: Vec<i32>, base: Option<&'a Namespace<'a>>, arena: &'a Arena<Namespace<'a>>) -> i32 {
    0
}