use crate::lexer::Token;

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn next(&mut self) -> Option<&Token> {
        self.pos += 1;
        self.tokens.get(self.pos - 1)
    }

    fn prev(&mut self) -> Option<&Token> {
        self.pos -= 1;
        self.tokens.get(self.pos)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek_nonspace(&self) -> Option<&Token> {
        let mut a = 0;
        while let Some(Token::Space(_)) = self.tokens.get(self.pos + a) {
            a += 1;
        }
        self.tokens.get(self.pos + a)
    }

    fn space(&mut self) -> usize {
        if let Some(Token::Space(s)) = self.peek() {
            let s = *s;
            self.pos += 1;
            s
        } else {
            0
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_op(0)
        // let lhs = self.parse_term()?;

        // let lspace = self.space();

        // Ok(match self.next() {
        //     Some(op @ (Token::Plus | Token::Minus)) => {
        //         let rspace = self.space();
        //         let rhs = self.parse_term()?;
        //         Expr::Bin {
        //             lhs: Box::new(lhs),
        //             rhs: Box::new(rhs),
        //             op,
        //             lspace,
        //             rspace,
        //         }
        //     }

        //     _ => lhs,
        // })
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        Ok(match self.next() {
            Some(Token::Number(n)) => Expr::Number(*n),
            Some(Token::String(s)) => Expr::String(s.clone()),
            Some(Token::True) => Expr::Bool(true),
            Some(Token::False) => Expr::Bool(false),

            Some(Token::Oparen) => {
                let lspace = self.space();
                let expr = Box::new(self.parse_expr()?);
                let rspace = self.space();
                self.expect(Token::Cparen)?;
                Expr::Parenthesized {
                    expr,
                    lspace,
                    rspace,
                }
            }

            Some(Token::If) => {
                let space1 = self.space();

                let cond = Box::new(self.parse_expr()?);

                let space2 = self.space();

                self.expect(Token::Then)?;

                let then = Box::new(self.parse_expr()?);

                let space3 = self.space();
                self.expect(Token::Else)?;
                let space4 = self.space();

                let else_ = Box::new(self.parse_expr()?);
                
                Expr::If {
                    cond,
                    then,
                    else_,
                    space: [space1, space2, space3, space4],
                }
            }

            _ => return Err("invalid expression".to_string()),
        })
    }

    pub fn parse_op(&mut self, prec: usize) -> Result<Expr, String> {
        let next_prec = prec + 1;

        let mut left = if next_prec <= MAX_PREC {
            self.parse_op(next_prec)?
        } else {
            self.parse_term()?
        };

        let op = match self.peek_nonspace() {
            Some(a) => a,
            None => return Ok(left),
        }
        .clone();

        if is_op(&op) && precedence(&op) == prec {
            let lspace = self.space();
            self.next();
            let rspace = self.space();

            let right = self.parse_op(prec)?;
            left = Expr::Bin {
                lhs: Box::new(left),
                rhs: Box::new(right),
                op,
                lspace,
                rspace,
            };
        }
        Ok(left)
    }

    fn expect(&mut self, token: Token) -> Result<(), String> {
        match self.next() {
            Some(t) if t == &token => Ok(()),
            Some(t) => Err(format!("expected {:?}, got {:?}", token, t)),
            None => Err("unexpected end of input".to_string()),
        }
    }
}

fn is_op(tok: &Token) -> bool {
    matches!(
        tok,
        Token::Plus
            | Token::Minus
            | Token::Mult
            | Token::Div
            | Token::And
            | Token::Or
            | Token::Eq
            | Token::Neq
            | Token::Gt
            | Token::Lt
    )
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(u16),
    String(String),
    Bool(bool),
    Bin {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        op: Token,
        lspace: usize,
        rspace: usize,
    },
    Parenthesized {
        expr: Box<Expr>,
        lspace: usize,
        rspace: usize,
    },

    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
        space: [usize; 4],
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<(usize, Expr, usize), String> {
    let mut parser = Parser { tokens, pos: 0 };
    let lspace = parser.space();
    let expr = parser.parse_expr()?;
    let rspace = parser.space();
    Ok((lspace, expr, rspace))
}

const MAX_PREC: usize = 1;

fn precedence(op: &Token) -> usize {
    match op {
        Token::And | Token::Or => 0,
        Token::Eq | Token::Neq | Token::Gt | Token::Lt => 0,
        Token::Plus | Token::Minus => 2,
        Token::Mult | Token::Div => 3,
        _ => 0,
    }
}
