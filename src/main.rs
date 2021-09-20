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
    debug_tokens(&token);

    let node = parse(token);

    codegen(node);
    return;
}

// Debug tokens which tokenizer generate.
fn debug_tokens(tokens: &Vec<Token>) {
    let mut count = 0;
    println!("////////TOKEN DEBUG START////////");
    for tok in tokens.iter() {
        match tok.kind {
            tokenize::TokenKind::NUM => {
                println!("index: {}, kind: {}, val: {}", count, tok.kind, tok.value,)
            }
            tokenize::TokenKind::PUNCT => {
                println!("index: {}, kind: {}, char: {}", count, tok.kind, tok.char,)
            }
            _ => {
                println!("index: {}, kind: {}", count, tok.kind,)
            }
        }
        count += 1;
    }
    println!("////////TOKEN DEBUG END////////");
}
