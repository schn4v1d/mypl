use combine::{
    between, choice, many1, parser, satisfy, token, EasyParser, ParseError, Parser, Stream,
};

use crate::{
    expr::Expr,
    primitives::{PrimitiveFunction, PrimitiveMonadicOperator},
    token::Token,
};

pub fn parse(input: &[Token]) -> Vec<Expr> {
    many1(expr()).easy_parse(input).map(|x| x.0).unwrap()
}

fn parenthesized<I>() -> impl Parser<I, Output = Expr>
where
    I: Stream<Token = Token>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    between(token(Token::LParens), token(Token::RParens), many1(expr()))
        .map(|es| Expr::Isolated(es))
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

fn primitive<I>() -> impl Parser<I, Output = Expr>
where
    I: Stream<Token = Token>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    choice((
        token(Token::Plus).map(|_| Expr::PrimitiveFunction(PrimitiveFunction::Plus)),
        token(Token::Minus).map(|_| Expr::PrimitiveFunction(PrimitiveFunction::Minus)),
        token(Token::Times).map(|_| Expr::PrimitiveFunction(PrimitiveFunction::Times)),
        token(Token::Divide).map(|_| Expr::PrimitiveFunction(PrimitiveFunction::Divide)),
        token(Token::TildeDiaeresis)
            .map(|_| Expr::PrimitiveMonadicOperator(PrimitiveMonadicOperator::Commute)),
    ))
}
