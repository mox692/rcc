#![feature(map_try_insert)]
#![feature(core_panic)]

mod codegen;
mod intermediate_process;
mod parse;
mod tokenize;

use codegen::codegen;
use intermediate_process::intermediate_process;
use parse::debug_functions;
use parse::debug_nodes;
use parse::parse;
use std::env;
use tokenize::debug_tokens;
use tokenize::tokenize;
use tokenize::NewTokenReader;
use tokenize::Token;

fn main() {
    // [args]
    // arg[1] -> source input.
    // arg[2] -> debug flag.
    let mut args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    let mut debug_flag = false;

    // (tokenizeがしやすくなるため)終端文字を加える.
    args[1].push('\0');
    if arg_len == 3 {
        debug_flag = if args[2].eq("true") { true } else { false };
    }

    let input: &String = &args[1];
    let token: Vec<Token> = tokenize(input);
    // debug token.
    debug_tokens(debug_flag, &token);

    let mut tokenReader = NewTokenReader(token);

    let mut functions = parse(&mut tokenReader);
    // debug node.

    debug_functions(debug_flag, &functions);

    functions = intermediate_process(functions);

    // generate assembly
    codegen(functions);

    return;
}
