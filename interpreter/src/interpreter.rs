use typed_arena::Arena;

use crate::parser::Expr;

struct Namespace<'a> {
    binding: (i32, &'a Expr),
    base: Option<&'a Namespace<'a>>,
}

pub fn interpret(expr: &Expr) -> i32 {
    eval(expr, None)
}

fn eval<'a>(expr: &'a Expr, base: Option<&'a Namespace<'a>>) -> i32 {
    match expr {
        Expr::INT(nbr) => *nbr,
        Expr::FUNC(exprs) => {
            let op = eval(&exprs[0], base);
            match op {
                0 => {
                    // Bind
                    let id = eval(&exprs[1], base);
                    let ns = Namespace {
                        binding: (id, &exprs[2]),
                        base: base,
                    };
                    eval(&exprs[3], Some(&ns))
                }
                1 => {
                    // If
                    let cond = eval(&exprs[1], base);
                    let ind = 2 + (cond <= 0) as usize;
                    eval(&exprs[ind], base)
                }
                2 => {
                    // Print
                    let nbr = eval(&exprs[1], base);
                    print!("{}", nbr as u8 as char);
                    nbr
                }
                3 => {
                    // Sub
                    let a = eval(&exprs[1], base);
                    let b = eval(&exprs[2], base);
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
                        let val = eval(expr, base);
                        let argexpr = exprarena.alloc(Expr::INT(val));
                        ns = nsarena.alloc(Namespace {
                            binding: (-id, argexpr),
                            base: Some(ns),
                        });
                    }

                    eval(fexpr, Some(ns))
                }
            }
        }
    }
}

fn lookup<'a>(id: i32, base: Option<&'a Namespace<'a>>) -> &'a Namespace<'a> {
    let ns = base.expect(&format!("Could not find {}!", id));
    if ns.binding.0 == id {
        ns
    } else {
        lookup(id, ns.base)
    }
}
