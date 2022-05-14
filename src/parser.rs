use combine::{
    between, choice, parser, satisfy, token, EasyParser, ParseError, Parser, RangeStream, Stream,
};

use crate::{expr::Expr, token::Token};

pub fn read(input: &[Token]) {
    expr().easy_parse(input).unwrap();
}

fn parenthesized<I>() -> impl Parser<I, Output = Expr>
where
    I: Stream<Token = Token>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    between(token(Token::LParens), token(Token::RParens), expr())
        .map(|e| Expr::Isolated(Box::new(e)))
}

fn expr<I>() -> impl Parser<I, Output = Expr>
where
    I: Stream<Token = Token>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    expr_()
}

parser! {
    #[inline]
    fn expr_[Input]()(Input) -> Expr
    where [ Input: Stream<Token = Token> ]
    {
        choice((
            parenthesized(),
            scalar(),
            primitive(),
        ))
    }
}

fn scalar<I>() -> impl Parser<I, Output = Expr>
where
    I: Stream<Token = Token>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    let integer = satisfy(|tok| matches!(tok, Token::Integer(_))).map(|tok| match tok {
        Token::Integer(n) => Expr::Integer(n),
        _ => unreachable!(),
    });

    let float = satisfy(|tok| matches!(tok, Token::Float(_))).map(|tok| match tok {
        Token::Float(n) => Expr::Float(n),
        _ => unreachable!(),
    });

    choice((integer, float))
}
