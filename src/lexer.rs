use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r#"\d+"#, |lex| lex.slice().parse::<u16>().unwrap())]
    Number(u16),
    #[regex(r#"\w*"(?:\\.|[^\\"])*"|'(?:\\.|[^\\'])*'"#, |lex| {
        let s = lex.slice();
        s[1..(s.len() - 1)].to_string()
    })]
    String(String),

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Mult,
    #[token("/")]
    Div,
    // boolean
    #[token("&")]
    And,
    #[token("|")]
    Or,
    #[token(">")]
    Gt,
    #[token("<")]
    Lt,
    #[token("=")]
    Eq,
    #[token("!")]
    Neq,


    #[token("true")]
    True,
    #[token("false")]
    False,

    #[token("if")]
    If,
    #[token("then")]
    Then,
    #[token("else")]
    Else,

    #[token("(")]
    Oparen,

    #[token(")")]
    Cparen,

    #[regex(" +", |lex| lex.slice().len())]
    Space(usize),
    #[token("\n")]
    Newline,

    // #[regex(r"[ \t\f\n\r]+|/\*[^*]*\*(([^/\*][^\*]*)?\*)*/|//[^\n]*", logos::skip)]
    #[error]
    Error,
}

pub fn lex(code: &str) -> Vec<Token> {
    // replace all tabs with spaces
    let code = code.replace("\t", "    ").replace("\r", "");

    let lexer = Token::lexer(&code);
    lexer.collect::<Vec<_>>()
}
