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

struct CodeLabel {
    cur_index: usize,
}
impl CodeLabel {
    fn new() -> Self {
        return CodeLabel { cur_index: 0 };
    }
    fn cur_label_index(&self) -> usize {
        return self.cur_index;
    }
    fn increment(&mut self) {
        self.cur_index += 1;
    }
}

pub fn codegen(nodes: &Vec<Box<Node>>) {
    let mut lv = LocalVariable::new();
    let mut cl = CodeLabel::new();
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
        gen(node, &mut f, &mut lv, &mut cl);
    }

    writeln!(f, "pop %rax");
    writeln!(f, "mov %rbp, %rsp");
    writeln!(f, "pop %rbp");
    writeln!(f, "ret");
}

// 引数で渡されたNodeを展開して、その評価結果をstack topにpushする.
fn gen(node: &Node, f: &mut File, lv: &mut LocalVariable, cl: &mut CodeLabel) {
    /*
        gen from unary node.
    */
    if node.kind == NodeKind::ND_NUM {
        writeln!(f, "push ${}", node.val);
        return;
    }
    // MEMO: このnodeだけ例外的にepilogueもコードに入れている.
    if node.kind == NodeKind::ND_RETURN {
        // evaluate expr.
        gen(node.l.as_ref().unwrap().as_ref(), f, lv, cl);
        writeln!(f, "pop %rax");
        writeln!(f, "mov %rbp, %rsp");
        writeln!(f, "pop %rbp");
        // MEMO: 評価はreturn後のexprがされるが、
        // コンパイラが吐くアセンブリコードは自体はreturn以降のコードも出力される仕様にしている.
        writeln!(f, "ret");
        return;
    }
    if node.kind == NodeKind::ND_EXPR || node.kind == NodeKind::ND_STMT {
        gen(node.l.as_ref().unwrap().as_ref(), f, lv, cl);
        return;
    }
    if node.kind == NodeKind::ND_IDENT {
        // node.strに対応するmemoryからデータを取ってきて、stackにpushする.
        writeln!(f, "lea -{}(%rbp), %rax", lv.get_offset(node.str.clone()));
        // TODO: get_addrに(変数が見つからなかった際の)errハンドリングもやらせる
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

        gen(node.r.as_ref().unwrap().as_ref(), f, lv, cl);
        writeln!(f, "pop %rax");
        writeln!(f, "pop %rdi");
        writeln!(f, "mov %rax, (%rdi)");
        return;
    }
    if node.kind == NodeKind::ND_IFSTMT {
        cl.cur_index += 1;
        let if_node = node.if_node.as_ref().unwrap();
        let elsif_node = node.elsif_node.as_ref();
        let else_node = node.else_node.as_ref();

        // call if
        gen(if_node, f, lv, cl);

        if !elsif_node.is_none() {
            writeln!(f, ".L{}:", cl.cur_label_index());
            cl.cur_index += 1;
            gen(elsif_node.unwrap(), f, lv, cl);
        }

        if !else_node.is_none() {
            writeln!(f, ".L{}:", cl.cur_label_index());
            cl.cur_index += 1;
            gen(else_node.unwrap(), f, lv, cl);
        }

        writeln!(f, ".L{}:", cl.cur_label_index());

        return;
    }
    if node.kind == NodeKind::ND_IF || node.kind == NodeKind::ND_ELSIF {
        gen(node.l.as_ref().unwrap(), f, lv, cl);
        writeln!(f, "pop %rax");
        writeln!(f, "mov $1, %rdi");
        writeln!(f, "cmp %rdi, %rax");
        writeln!(f, "jne .L{}", cl.cur_label_index());
        // stmt
        gen(node.r.as_ref().unwrap(), f, lv, cl);
        return;
    }
    if node.kind == NodeKind::ND_ELSE {
        gen(node.l.as_ref().unwrap(), f, lv, cl);
        return;
    }
    if node.kind == NodeKind::ND_IFCOND {
        gen(node.l.as_ref().unwrap(), f, lv, cl);
        return;
    }

    // other binary operation.
    gen(node.l.as_ref().unwrap().as_ref(), f, lv, cl);
    gen(node.r.as_ref().unwrap().as_ref(), f, lv, cl);

    writeln!(f, "pop %rdi"); // right side.
    writeln!(f, "pop %rax"); // left side.

    match node.kind {
        // 四則演算.
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
        // 比較演算.
        NodeKind::ND_EQ => {
            writeln!(f, "cmp %rdi, %rax");
            writeln!(f, "sete %al");
            writeln!(f, "movzb %al, %rax");
        }
        NodeKind::ND_NEQ => {
            writeln!(f, "cmp %rdi, %rax");
            writeln!(f, "setne %al");
            writeln!(f, "movzb %al, %rax");
        }
        NodeKind::ND_LE => {
            writeln!(f, "cmp %rdi, %rax");
            writeln!(f, "setle %al");
            writeln!(f, "movzb %al, %rax");
        }
        NodeKind::ND_LT => {
            writeln!(f, "cmp %rdi, %rax");
            writeln!(f, "setl %al");
            writeln!(f, "movzb %al, %rax");
        }
        NodeKind::ND_BE => {
            writeln!(f, "cmp %rax, %rdi");
            writeln!(f, "setle %al");
            writeln!(f, "movzb %al, %rax");
        }
        NodeKind::ND_BT => {
            writeln!(f, "cmp %rax, %rdi");
            writeln!(f, "setl %al");
            writeln!(f, "movzb %al, %rax");
        }
        _ => {}
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
