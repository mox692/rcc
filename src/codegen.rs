use std::error::Error;

// ref: https://keens.github.io/blog/2018/12/08/rustnomoju_runotsukaikata_2018_editionhan/
use crate::tokenize::Token;

// stringをnumに変換する
pub fn string_to_num(string: &String) -> i32 {
    match string.parse::<i32>() {
        Ok(x) => x,
        Err(e) => panic!(e),
    }
}

pub fn codegen(token: Vec<Token>) {
    // put start up.
    println!(".text");
    println!(".global main");
    println!("main:");
    println!("pushq %rbp");
    println!("movq %rsp, %rbp");

    println!("movq $4, %rax");

    println!("pop %rbp");
    println!("ret");
}
