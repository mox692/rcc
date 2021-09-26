use crate::parse::NodeKind;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

// ref: https://keens.github.io/blog/2018/12/08/rustnomoju_runotsukaikata_2018_editionhan/
use crate::parse::Node;

struct LocalVariable {
    pub count: usize,
    pub local_vals: HashMap<String, usize>,
}
impl LocalVariable {
    fn new() -> LocalVariable {
        return LocalVariable {
            count: 0,
            local_vals: HashMap::new(),
        };
    }
    fn register_symbol(&mut self, symbol: String, offset: usize) {
        self.local_vals.insert(symbol, offset);
        return;
    }
    fn incre_count(&mut self) {
        self.count += 1;
    }
    fn new_offset(&mut self, symbol: String) -> usize {
        let new_offset = (self.count + 1) * 8;
        self.incre_count();
        self.register_symbol(symbol, new_offset);
        return new_offset;
    }
    fn get_offset(&self, symbol: String) -> usize {
        match self.local_vals.get::<String>(&symbol) {
            None => {
                panic!("cannot find local val.")
            }
            Some(offset) => {
                return *offset;
            }
        }
    }
}

pub fn codegen(nodes: &Vec<Box<Node>>) {
    let mut lv = LocalVariable::new();
    let mut f = create_file("./gen.s");
    // put start up.
    writeln!(f, ".text");
    writeln!(f, ".global main");
    writeln!(f, "main:");
    writeln!(f, "pushq %rbp");
    writeln!(f, "movq %rsp, %rbp");

    // TODO: hard code
    writeln!(f, "sub $128, %rsp");

    // 各stmt毎にcodegen.
    for node in nodes {
        gen(node, &mut f, &mut lv);
    }

    writeln!(f, "pop %rax");
    writeln!(f, "mov %rbp, %rsp");
    writeln!(f, "pop %rbp");
    writeln!(f, "ret");
}

// 引数で渡されたNodeを展開して、その評価結果をstack topにpushする.
fn gen(node: &Node, f: &mut File, lv: &mut LocalVariable) {
    /*
        gen from unary node.
    */
    if node.kind == NodeKind::ND_NUM {
        writeln!(f, "push ${}", node.val);
        return;
    }
    // EXPR, STMTは展開してやるだけ.
    if node.kind == NodeKind::ND_EXPR || node.kind == NodeKind::ND_STMT {
        gen(node.l.as_ref().unwrap().as_ref(), f, lv);
        return;
    }
    if node.kind == NodeKind::ND_IDENT {
        // node.strに対応するmemoryからデータを取ってきて、stackにpushする.
        writeln!(f, "lea -{}(%rbp), %rax", lv.get_offset(node.str.clone()));
        //       TODO: get_addrに(変数が見つからなかった際の)errハンドリングもやらせる
        writeln!(f, "mov (%rax), %rax");
        writeln!(f, "push %rax");
        return;
    }

    /*
        gen from binary node.
    */
    // TODO: ここのpathは評価結果をstackにpushしないから、
    //       assignで終了するような入力ソースコードを受け取ると、
    //       変な終了コードになりそう.(まあ、さしあたりはそんなことは気にしない.)
    if node.kind == NodeKind::ND_ASSIGN {
        // 左辺のstrと紐付けた形でstack上にデータ領域を確保.
        // -> ND_EXPRのcodeを生成.
        writeln!(
            f,
            "lea -{}(%rbp), %rax",
            lv.new_offset(node.l.as_ref().unwrap().str.clone())
        );
        writeln!(f, "push %rax");

        // 右辺の値をstackに積む.
        gen(node.r.as_ref().unwrap().as_ref(), f, lv);
        writeln!(f, "pop %rax"); // 右辺値(val)
        writeln!(f, "pop %rdi"); // 左辺値(addr)
        writeln!(f, "mov %rax, (%rdi)");

        return;
    }

    gen(node.l.as_ref().unwrap().as_ref(), f, lv);
    gen(node.r.as_ref().unwrap().as_ref(), f, lv);

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
