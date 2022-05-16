use std::io::{stdout, BufRead, Write};

use logos::Logos;
use parser::parse;

use crate::{eval_tree::build_tree, token::Token, value::eval};

pub mod eval_tree;
pub mod expr;
pub mod parser;
pub mod primitives;
pub mod token;
pub mod value;

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    print!("   ");
    out.flush().unwrap();

    for line in stdin.lock().lines() {
        let tokens = Token::lexer(&line.unwrap()).collect::<Vec<_>>();

        // println!("{:#?}", tokens);

        let expressions = parse(&tokens);

        // println!("{:#?}", expressions);

        let tree = build_tree(expressions);

        // println!("{:#?}", tree);

        let result = eval(&tree);

        // println!("{:#?}", result);

        println!("{}", result);
        print!("   ");
        out.flush().unwrap();
    }
}
