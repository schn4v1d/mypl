use logos::{Lexer, Logos};

#[derive(Debug, Clone, Copy, PartialEq, Logos)]
pub enum Token {
    // Atoms
    #[regex(r"¯?[0-9]+", parse_integer)]
    Integer(i64),
    #[regex(r"¯?[0-9]+\\.[0-9]+", parse_float)]
    Float(f64),

    // Symbols
    #[token("(")]
    LParens,
    #[token(")")]
    RParens,

    // Primitive Functions
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("×")]
    Times,
    #[token("÷")]
    Divide,

    // Primitive Operators
    #[token("⍨")]
    TildeDiaeresis,

    // Misc
    #[error]
    #[regex(r"[ \t]+", logos::skip)]
    Error,
}

impl Token {
    fn is_function(&self) -> bool {
        matches!(
            self,
            Token::Plus | Token::Minus | Token::Times | Token::Divide
        )
    }

    fn is_operator(&self) -> bool {
        matches!(self, Token::TildeDiaeresis)
    }
}

fn parse_integer(lex: &mut Lexer<Token>) -> Option<i64> {
    let slice = lex.slice();

    let (negative, n): (bool, i64) = if slice.starts_with('¯') {
        (true, slice[1..].parse().ok()?)
    } else {
        (false, slice.parse().ok()?)
    };

    Some(if negative { -n } else { n })
}

fn parse_float(lex: &mut Lexer<Token>) -> Option<f64> {
    let slice = lex.slice();

    let (negative, n): (bool, f64) = if slice.starts_with('¯') {
        (true, slice[1..].parse().ok()?)
    } else {
        (false, slice.parse().ok()?)
    };

    Some(if negative { -n } else { n })
}
