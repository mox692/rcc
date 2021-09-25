use std::error::Error;

// ref: https://keens.github.io/blog/2018/12/08/rustnomoju_runotsukaikata_2018_editionhan/
use crate::parse::Node;

// stringをnumに変換する
pub fn string_to_num(string: &String) -> i32 {
    match string.parse::<i32>() {
        Ok(x) => x,
        Err(e) => panic!(e),
    }
}

pub fn codegen(node: &Node) {
    let mut f = create_file("./gen.s");
    // put start up.
    writeln!(f, ".text");
    writeln!(f, ".global main");
    writeln!(f, "main:");
    writeln!(f, "pushq %rbp");
    writeln!(f, "movq %rsp, %rbp");

    writeln!(f, "movq $4, %rax");

    writeln!(f, "pop %rbp");
    writeln!(f, "ret");
}

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn create_file(path: &str) -> File {
    match OpenOptions::new().write(true).create(true).open(path) {
        Ok(f) => f,
        Err(e) => panic!(e),
    }
}
