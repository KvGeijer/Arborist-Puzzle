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
                    println!("Char: {}", nbr as u8 as char);
                    println!("i32: {}", nbr);
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
                    let mut ns = lookup(nbr, base);
                    let fexpr = ns.binding.1;

                    // Temp arena for the args
                    let exprarena = Arena::new();
                    let nsarena = Arena::new();
                    
                    for (expr, id) in exprs[1..].into_iter().zip(1..) {
                        let val = eval(expr, base, arena);
                        let argexpr = exprarena.alloc(Expr::INT(val));
                        ns = nsarena.alloc(Namespace{binding: (-id, argexpr), base: Some(ns)});
                    }

                    eval(fexpr, Some(ns), arena)
                }
            }
        }
    }
}

fn lookup<'a>(id: i32, base: Option<&'a Namespace<'a>>) -> &'a Namespace<'a> {
    let ns = base.expect("Could not find nbr!");
    if ns.binding.0 == id {
        ns
    } else {
        lookup(id, ns.base)
    }
}