#![feature(map_try_insert)]
#![feature(core_panic)]

mod codegen;
mod errors;
mod intermediate_process;
mod parse;
mod tokenize;

use codegen::codegen;
use intermediate_process::intermediate_process;
use parse::debug_functions;
use parse::parse;
use std::env;
use tokenize::debug_tokens;
use tokenize::tokenize;
use tokenize::NewTokenReader;
use tokenize::Token;
// use once_cell::sync::Lazy;
// use std::sync::Mutex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input: String = args[1].clone();

    let arg_len = args.len();
    let mut debug_flag = false;

    // (tokenizeがしやすくなるため)終端文字を加える.
    input.push('\0');
    if arg_len == 3 {
        debug_flag = if args[2].eq("true") { true } else { false };
    }

    let token: Vec<Token> = tokenize(input);
    // debug token.
    debug_tokens(debug_flag, &token);

    let mut token_reader = NewTokenReader(token);

    let mut functions = parse(&mut token_reader);

    // debug node.
    debug_functions(debug_flag, &functions);

    functions = intermediate_process(functions);

    // generate assembly
    codegen(functions);

    return;
}
