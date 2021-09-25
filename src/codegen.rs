use crate::parse::NodeKind;
use std::fs::File;
use std::io::prelude::*;

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

    gen(node, &mut f);

    writeln!(f, "pop %rax");
    writeln!(f, "pop %rbp");
    writeln!(f, "ret");
}

fn gen(node: &Node, f: &mut File) {
    if node.kind == NodeKind::ND_NUM {
        writeln!(f, "push ${}", node.val);
        return;
    }

    gen(node.l.as_ref().unwrap().as_ref(), f);
    gen(node.r.as_ref().unwrap().as_ref(), f);

    writeln!(f, "pop %rdi"); // right side.
    writeln!(f, "pop %rax"); // left side.

    match node.kind {
        NodeKind::ND_ADD => {
            writeln!(f, "add %rdi, %rax");
        }
        NodeKind::ND_SUB => {
            writeln!(f, "sub %rdi, %rax");
        }
        NodeKind::ND_MUL => {
            writeln!(f, "imul %rdi, %rax");
        }
        NodeKind::ND_DIV => {
            writeln!(f, "cqo");
            writeln!(f, "idiv %rdi");
        }
        _ => {
            panic!("Unsapported node kind found");
        }
    }
    writeln!(f, "push %rax");
}

fn create_file(path: &str) -> File {
    let f = match File::create(path) {
        Ok(f) => f,
        Err(e) => panic!(e),
    };
    return f;
}
