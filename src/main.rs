mod codegen;
mod parse;
mod tokenize;

use codegen::codegen;
use parse::parse;
use std::env;
use tokenize::tokenize;
use tokenize::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("argc is {}, not 2", args.len());
        return;
    }
    let input: &String = &args[1];

    let token: Vec<Token> = tokenize(input);

    let mut count = 0;
    for tok in token.iter() {
        println!("count: {}, kind: {}", count, tok.kind);
        count += 1;
    }

    let node = parse(token);

    codegen(node);
    return;
}
