use logos::Logos;

use crate::token::Token;

mod expr;
mod token;
mod parser;
mod value;
mod primitives;

fn main() {
    let mut lex = Token::lexer("+1 2 3");

    while let Some(tok) = lex.next() {
        println!("{:?}", tok);
        println!("{:?}", lex.slice());
    }
}
