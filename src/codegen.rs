use crate::parse::NodeKind;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

// ref: https://keens.github.io/blog/2018/12/08/rustnomoju_runotsukaikata_2018_editionhan/
use crate::parse::Function;
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
    // new_offset gets symbol name given from codegenarator,
    // and returns offset from rbp.
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

pub fn codegen(function: &Function) {
    let nodes = &function.nodes;

    let mut lv = LocalVariable::new();
    let mut cl = CodeLabel::new();
    let mut f = create_file("./gen.s");
    // put start up.
    writeln!(f, ".text");
    writeln!(f, ".global main");
    writeln!(f, "main:");
    writeln!(f, "pushq %rbp");
    writeln!(f, "movq %rsp, %rbp");

    // MEMO: rspを下げるサイズは必ず16の倍数にならないといけないらしいので
    //       それ用に返す値を少しいじってる.
    writeln!(
        f,
        "sub ${}, %rsp",
        if function.lv_size % 2 == 0 {
            8 * function.lv_size
        } else {
            (function.lv_size + 1) * 8
        }
    );

    // 各stmt毎にcodegen.
    // TODO: 将来的には(Nodeというより)Function毎にcodegenをしていくイメージ.
    //       また、関数ごとに(上で書いている様な)prologue,epilogueの処理を入れる.
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
        // シンボル(node.str)に対応するアドレスからデータを取ってきて、stackにpushする.
        writeln!(f, "lea -{}(%rbp), %rax", lv.get_offset(node.str.clone()));
        // TODO: get_offsetに(変数が見つからなかった際の)errハンドリングもやらせる
        writeln!(f, "mov (%rax), %rax");
        writeln!(f, "push %rax");
        return;
    }
    if node.kind == NodeKind::ND_FNCALL {
        // TODO: まだ関数呼び出しができない.
        //       関数呼び出しを見つけると、codegeneratorは
        //       stackに0をpushする.(どんな関数呼び出しも0として評価される.)
        writeln!(f, "push $0");
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

    if node.kind == NodeKind::ND_STMT2 {
        let node_vec = node.stmts2.clone();
        let len = node.stmts2_len;
        let mut i = 0;
        loop {
            gen(node_vec[i].as_ref().unwrap(), f, lv, cl);
            i += 1;
            if len == i {
                break;
            }
        }
        return;
    }

    if node.kind == NodeKind::ND_FOR {
        // To prevent name crash, we assign unique
        // (as long as this for scope) label.
        let for_start_label = format!("L_FOR_START{}", cl.cur_label_index());
        let for_end_label = format!("L_FOR_END{}", cl.cur_label_index());

        // Assuming that for or if will be called recursively,
        // increment the label index at this timing.
        cl.cur_index += 1;

        gen(node.for_node_first_assign.as_ref().unwrap(), f, lv, cl);
        writeln!(f, ".{}:", for_start_label);
        gen(node.for_node_stmts.as_ref().unwrap(), f, lv, cl);
        gen(node.for_node_second_condition.as_ref().unwrap(), f, lv, cl);
        writeln!(f, "pop %rax");
        writeln!(f, "mov $1, %rdi");
        writeln!(f, "cmp %rdi, %rax");
        writeln!(f, "jne .{}", for_end_label);
        gen(node.for_node_third_expr.as_ref().unwrap(), f, lv, cl);
        writeln!(f, "jmp .{}", for_start_label);
        writeln!(f, ".{}:", for_end_label);
        return;
    }

    // NodeKind::ND_IFSTMT is the node that will be the entry
    // for all if statements. This block calls the ND_IF, ND_ELSIF,
    // and ND_ELSE statement codegen.
    if node.kind == NodeKind::ND_IFSTMT {
        let if_node = node.if_node.as_ref().unwrap();
        let elsif_node = node.elsif_node.as_ref();
        let else_node = node.else_node.as_ref();

        // codegen ND_IF
        gen(if_node, f, lv, cl);

        if !elsif_node.is_none() {
            gen(elsif_node.unwrap(), f, lv, cl);
        }

        if !else_node.is_none() {
            gen(else_node.unwrap(), f, lv, cl);
        }

        return;
    }
    if node.kind == NodeKind::ND_IF || node.kind == NodeKind::ND_ELSIF {
        cl.cur_index += 1;
        let mut i = cl.cur_label_index();
        gen(node.l.as_ref().unwrap(), f, lv, cl);
        writeln!(f, "pop %rax");
        writeln!(f, "mov $1, %rdi");
        writeln!(f, "cmp %rdi, %rax");

        writeln!(f, "jne .L{}", i);
        // stmt
        gen(node.r.as_ref().unwrap(), f, lv, cl);
        writeln!(f, ".L{}:", i);
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
