#![feature(map_try_insert)]
#![feature(core_panic)]

mod codegen;
mod errors;
mod intermediate_process;
mod parse;
mod tokenize;
use codegen::codegen;
use intermediate_process::intermediate_process;
use parse::{debug_functions, parse};
use tokenize::{debug_tokens, new_token_reader, tokenize, Token};

use clap::{App, Arg};
use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let matches = App::new("rcc")
        .author("Motoyuki Kimura")
        .about("A small C compiler")
        .arg(Arg::new("SOURCE").help("input source."))
        .arg(
            Arg::new("std")
                .long("std")
                .short('s')
                .help("get input by stdin."),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('d')
                .help("print debug info to stdout."),
        )
        .get_matches();

    let mut source_input: String;
    let mut debug_flag = false;

    // sourceを取得
    if matches.is_present("std") {
        source_input = matches
            .value_of("SOURCE")
            .unwrap_or_else(|| panic!("Source is not specified."))
            .to_string();
    } else {
        let path = matches
            .value_of("SOURCE")
            .unwrap_or_else(|| panic!("Source is not specified."))
            .to_string();
        source_input = fs::read_to_string(path)?;
    }
    // debug flagの確認
    if let Some(_) = matches.value_of("debug") {
        debug_flag = true
    }

    // 末尾に終端文字を入れておく
    source_input.push('\0');

    let token: Vec<Token> = tokenize(source_input);

    debug_tokens(debug_flag, &token);

    let mut token_reader = new_token_reader(token);

    let mut functions = parse(&mut token_reader);

    debug_functions(debug_flag, &functions);

    functions = intermediate_process(functions);

    // generate assembly
    codegen(functions);

    return Ok(());
}
