use std::cmp::{max, min};

use crate::lexer::Token;
use crate::parser::Expr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Number(u16),
    String(String),
    Bool(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

pub type Animation = Vec<String>;

pub struct AnimateResult {
    pub value: Value,
    pub anim: Animation,
    // last frame with spacing
    pub last: (usize, String, usize),
}

pub fn animate_eval(expr: Expr) -> Result<AnimateResult, String> {
    match expr {
        Expr::Number(n) => Ok(AnimateResult {
            value: Value::Number(n),
            anim: vec![],
            last: (0, Value::Number(n).to_string(), 0),
        }),
        Expr::String(s) => Ok(AnimateResult {
            value: Value::String(s.clone()),
            anim: vec![],
            last: (0, Value::String(s).to_string(), 0),
        }),
        Expr::Bool(b) => Ok(AnimateResult {
            value: Value::Bool(b),
            anim: vec![],
            last: (0, Value::Bool(b).to_string(), 0),
        }),

        Expr::Bin {
            lhs,
            rhs,
            op,
            lspace,
            rspace,
        } => {
            let AnimateResult {
                value: v1,
                anim: a1,
                last: (pre_space1, last1, post_space1),
            } = animate_eval(*lhs)?;

            let AnimateResult {
                value: v2,
                anim: a2,
                last: (pre_space2, last2, post_space2),
            } = animate_eval(*rhs)?;

            let result = eval_op(v1, v2, &op);

            let op = match &op {
                Token::Plus => "+",
                Token::Minus => "-",
                Token::Mult => "*",
                Token::Div => "/",

                Token::And => "&",
                Token::Or => "|",
                Token::Eq => "=",
                Token::Neq => "!",
                Token::Lt => "<",
                Token::Gt => ">",
                _ => unreachable!(),
            };

            let mut anim = vec![];
            let solve_anim_len = max(a1.len(), a2.len());
            for i in 0..solve_anim_len {
                let frame1 = if i < a1.len() {
                    a1[i].clone()
                } else {
                    format!(
                        "{}{}{}",
                        " ".repeat(pre_space1),
                        last1,
                        " ".repeat(post_space1)
                    )
                };

                let frame2 = if i < a2.len() {
                    a2[i].clone()
                } else {
                    format!(
                        "{}{}{}",
                        " ".repeat(pre_space2),
                        last2,
                        " ".repeat(post_space2)
                    )
                };

                anim.push(format!(
                    "{}{}{}{}{}",
                    frame1,
                    " ".repeat(lspace),
                    op,
                    " ".repeat(rspace),
                    frame2
                ));
            }

            let lspace = lspace + post_space1;
            let rspace = rspace + pre_space2;

            let anim_len = min(max(lspace, rspace) + 1, 10);

            for i in 0..anim_len {
                let p = 1.0 - ease_in(i as f64 / anim_len as f64);
                let l = (lspace as f64 * p) as usize;
                let r = (rspace as f64 * p) as usize;
                anim.push(
                    [
                        " ".repeat(pre_space1),
                        last1.clone(),
                        " ".repeat(l),
                        op.to_string(),
                        " ".repeat(r),
                        last2.clone(),
                        " ".repeat(post_space2),
                    ]
                    .concat(),
                );
            }

            anim.push(format!(
                "{}{}{}",
                " ".repeat(pre_space1 + 1),
                &result,
                " ".repeat(post_space1 + 1)
            ));

            let (fl, fr) = center(&mut anim);
            let last = format!("{}", &result);

            Ok(AnimateResult {
                value: result,
                anim,
                last: (fl + pre_space1 + 1, last, fr + post_space2 + 1),
            })
        }
        Expr::Parenthesized {
            expr,
            mut lspace,
            mut rspace,
        } => {
            let AnimateResult {
                value: v,
                anim: mut a,
                last: (pre_space, last, post_space),
            } = animate_eval(*expr)?;

            for f in a.iter_mut() {
                *f = format!("({}{}{})", " ".repeat(lspace), f, " ".repeat(rspace));
            }

            lspace += pre_space;
            rspace += post_space;

            let paren_anim_len = min(max(lspace, rspace) + 1, 10);

            for i in 0..paren_anim_len {
                let e = 1.0 - ease_in(i as f64 / paren_anim_len as f64);
                let l = (lspace as f64 * e) as usize;
                let r = (rspace as f64 * e) as usize;
                a.push(format!("({}{}{})", " ".repeat(l), &last, " ".repeat(r)));
            }
            let (left, right) = center(&mut a);
            Ok(AnimateResult {
                value: v,
                anim: a,
                last: (left + 1, last, right + 1),
            })
        }
        Expr::If {
            cond,
            then,
            else_,
            space,
        } => todo!(),
    }
}

fn eval_op(v1: Value, v2: Value, op: &Token) -> Value {
    use Token::*;

    match (v1, op, v2) {
        (Value::Number(n1), Plus, Value::Number(n2)) => Value::Number(n1 + n2),
        (Value::Number(n1), Minus, Value::Number(n2)) => Value::Number(n1 - n2),
        (Value::Number(n1), Mult, Value::Number(n2)) => Value::Number(n1 * n2),
        (Value::Number(n1), Div, Value::Number(n2)) => Value::Number(n1 / n2),
        (Value::String(s1), Plus, Value::String(s2)) => Value::String(s1 + &s2),

        (Value::Bool(b1), Or, Value::Bool(b2)) => Value::Bool(b1 || b2),
        (Value::Bool(b1), And, Value::Bool(b2)) => Value::Bool(b1 && b2),

        (v1, Eq, v2) => Value::Bool(v1 == v2),
        (v1, Neq, v2) => Value::Bool(v1 != v2),

        (Value::Number(n1), Gt, Value::Number(n2)) => Value::Bool(n1 > n2),
        (Value::Number(n1), Lt, Value::Number(n2)) => Value::Bool(n1 < n2),

        _ => panic!("dont"),
    }
}

fn center(anim: &mut Animation) -> (usize, usize) {
    if anim.is_empty() {
        return (0, 0);
    }
    let len = anim[0].len();
    let mut left = 0;
    let mut right = 0;
    for f in anim.iter_mut() {
        let f_len = f.len();
        if f_len < len {
            let diff = len - f_len;
            left = diff / 2;
            right = diff - left;
            *f = format!("{}{}{}", " ".repeat(left), f, " ".repeat(right));
        }
    }
    (left, right)
}

// fn ease_in_out(t: f64) -> f64 {
//     let t = t * 2.0;
//     if t < 1.0 {
//         0.5 * t * t
//     } else {
//         let t = t - 1.0;
//         0.5 * (t * t + 2.0)
//     }
// }

fn ease_in(t: f64) -> f64 {
    t * t
}
