use crate::{
    intermediate_process::{blockstr_to_identid, FunctionLocalVariable, FN_ARG_BLOC_STR},
    parse::{Function, Node, NodeKind},
};
use std::{fs::File, io::prelude::*};

// forやifでjmpする先のLabelを管理するstruct.
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
}

#[allow(unused_must_use)]
pub fn codegen(functions: Vec<Function>) {
    let mut output_file = create_file("./gen.s");
    writeln!(output_file, ".text");
    for f in functions.iter() {
        codegen_func(f.clone(), &mut output_file);
    }
}

#[allow(unused_must_use)]
pub fn codegen_func(function: Function, f: &mut File) {
    let root_node = &function.root_node;

    // let mut lv = LocalVariable::new();
    let mut lv = function.local_variable.clone();
    let mut cl = CodeLabel::new();

    // put start up.
    writeln!(f, ".global {}", function.fn_name);
    writeln!(f, "{}:", function.fn_name);
    writeln!(f, "pushq %rbp");
    writeln!(f, "movq %rsp, %rbp");

    // MEMO: rspを下げるサイズは必ず16の倍数にならないといけないらしいので
    //       それ用に返す値を少しいじってる.
    //       今はInt型しかないので、これで大丈夫。
    writeln!(
        f,
        "sub ${}, %rsp",
        if (function.lv_size + function.fn_args_size) % 16 == 0 {
            function.lv_size + function.fn_args_size
        } else {
            8 + function.lv_size + function.fn_args_size
        }
    );

    // 関数の引数をmemに配置する
    for (i, arg) in function.fn_args.iter().cloned().enumerate() {
        let reg = vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
        let ident_id =
            blockstr_to_identid(arg.sym.clone(), String::from(FN_ARG_BLOC_STR));

        let val = function
            .local_variable
            .get_val_offset_by_identid_recursively(ident_id)
            .unwrap_or_else(|| panic!("symbol: {} not found", arg.sym.clone()));

        writeln!(f, "mov %{}, -{}(%rbp)", reg[i], val.offset);
    }

    // 各stmt毎にcodegen.
    // TODO: 将来的には(Nodeというより)Function毎にcodegenをしていくイメージ.
    //       また、関数ごとに(上で書いている様な)prologue,epilogueの処理を入れる.
    for node in root_node.fn_blocks.clone() {
        gen(&node, f, &mut lv, &mut cl);
    }
    writeln!(f, "pop %rax");
    writeln!(f, "mov %rbp, %rsp");
    writeln!(f, "pop %rbp");
    writeln!(f, "ret");
}

// 引数で渡されたNodeを展開して、その評価結果をstack topにpushする.
#[allow(unused_must_use)]
fn gen(node: &Node, f: &mut File, lv: &mut FunctionLocalVariable, cl: &mut CodeLabel) {
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
        if let Some(val) = lv.get_val_offset_by_identid_recursively(blockstr_to_identid(
            node.str.clone(),
            node.block_str.clone(),
        )) {
            writeln!(f, "lea -{}(%rbp), %rax", val.offset);
            writeln!(f, "mov (%rax), %rax");
            writeln!(f, "push %rax");
            return;
        }

        // 関数の引数を一応調べる(必要ないかも)
        let ident_id =
            blockstr_to_identid(node.str.clone(), String::from(FN_ARG_BLOC_STR));
        if let Some(val) = lv.get_val_offset_by_identid_recursively(ident_id) {
            writeln!(f, "lea -{}(%rbp), %rax", val.offset);
            writeln!(f, "mov (%rax), %rax");
            writeln!(f, "push %rax");
            return;
        } else {
            panic!("sym :{} not found.", node.str.clone())
        }
    }
    if node.kind == NodeKind::ND_PTR_REF {
        let src_node = node.ptr_ref_ident.clone().unwrap().as_ref().clone();
        let ident_id =
            blockstr_to_identid(src_node.str.clone(), src_node.block_str.clone());

        if let Some(val) = lv.get_val_offset_by_identid_recursively(ident_id) {
            writeln!(f, "leaq -{}(%rbp), %rax", val.offset);
            writeln!(f, "push %rax");
            return;
        } else {
            panic!("sym :{} not found.", src_node.str.clone())
        }
    }
    if node.kind == NodeKind::ND_PTR_DEREF {
        let src_node = node.ptr_deref_ident.clone().unwrap().as_ref().clone();
        let ident_id =
            blockstr_to_identid(src_node.str.clone(), src_node.block_str.clone());

        if let Some(val) = lv.get_val_offset_by_identid_recursively(ident_id) {
            writeln!(f, "mov -{}(%rbp), %rax", val.offset);
            writeln!(f, "mov (%rax), %rax");
            writeln!(f, "push %rax");
            return;
        } else {
            panic!("sym :{} not found.", node.str.clone())
        }
    }
    if node.kind == NodeKind::ND_FNCALL {
        let reg = vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
        // 引数をregisterにおく
        for (i, arg) in node.fn_call_args.iter().cloned().enumerate() {
            gen(arg.val.unwrap().as_ref(), f, lv, cl);
            writeln!(f, "pop %rax");
            writeln!(f, "mov %rax, %{}", reg[i]); // arg1
        }

        writeln!(f, "call {}", node.fn_name);
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
        // 右辺のoffsetをs取得
        let val = lv
            .get_val_offset_by_identid_recursively(blockstr_to_identid(
                node.l.as_ref().unwrap().str.clone(),
                node.l.as_ref().unwrap().block_str.clone(),
            ))
            .unwrap_or_else(|| panic!("Undecleare symbol!!"));
        // 左辺のstrと紐付けた形でstack上にデータ領域を確保.
        // -> ND_EXPRのcodeを生成.
        // TODO: getoffsetで、identIDを入れたらoffsetが出て9両に
        writeln!(f, "lea -{}(%rbp), %rax", val.offset);
        writeln!(f, "push %rax");

        gen(node.r.as_ref().unwrap().as_ref(), f, lv, cl);
        writeln!(f, "pop %rax");
        writeln!(f, "pop %rdi");
        writeln!(f, "mov %rax, (%rdi)");
        return;
    }
    if node.kind == NodeKind::ND_BLOCK {
        let node_vec = node.block_stmts.clone();
        for node in node_vec.iter() {
            gen(node, f, lv, cl);
        }
        return;
    }
    if node.kind == NodeKind::ND_STMT2 {
        gen(node.l.as_ref().unwrap().as_ref(), f, lv, cl);
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
        gen(node.for_node_second_condition.as_ref().unwrap(), f, lv, cl);
        writeln!(f, "pop %rax");
        writeln!(f, "mov $1, %rdi");
        writeln!(f, "cmp %rdi, %rax");
        writeln!(f, "jne .{}", for_end_label);
        gen(node.for_node_stmts.as_ref().unwrap(), f, lv, cl);
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
        let i = cl.cur_label_index();
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

    if node.kind == NodeKind::ND_DECL {
        // TODO: declnにblockstrがひっついている構造
        let ident_id = blockstr_to_identid(
            node.l.as_ref().unwrap().str.clone(),
            node.block_str.clone(),
        );
        let val = lv
            .get_val_offset_by_identid(ident_id.clone())
            .unwrap_or_else(|| {
                println!("ident_id: {}", ident_id.clone());
                panic!("Not Found!!")
            })
            .clone();

        writeln!(f, "lea -{}(%rbp), %rax", val.offset);
        writeln!(f, "push %rax");

        gen(node.r.as_ref().unwrap().as_ref(), f, lv, cl);
        writeln!(f, "pop %rax");
        writeln!(f, "pop %rdi");
        writeln!(f, "mov %rax, (%rdi)");
        return;
    }

    // other binary operation.
    gen(node.l.as_ref().unwrap().as_ref(), f, lv, cl);
    gen(node.r.as_ref().unwrap().as_ref(), f, lv, cl);

    writeln!(f, "pop %rdi"); // right side.
    writeln!(f, "pop %rax"); // left side.

    match node.kind {
        // 四則演算.
        | NodeKind::ND_ADD => {
            writeln!(f, "add %rdi, %rax");
        }
        | NodeKind::ND_SUB => {
            writeln!(f, "sub %rdi, %rax");
        }
        | NodeKind::ND_MUL => {
            writeln!(f, "imul %rdi, %rax");
        }
        | NodeKind::ND_DIV => {
            writeln!(f, "cqo");
            writeln!(f, "idiv %rdi");
        }
        // 比較演算.
        | NodeKind::ND_EQ => {
            writeln!(f, "cmp %rdi, %rax");
            writeln!(f, "sete %al");
            writeln!(f, "movzb %al, %rax");
        }
        | NodeKind::ND_NEQ => {
            writeln!(f, "cmp %rdi, %rax");
            writeln!(f, "setne %al");
            writeln!(f, "movzb %al, %rax");
        }
        | NodeKind::ND_LE => {
            writeln!(f, "cmp %rdi, %rax");
            writeln!(f, "setle %al");
            writeln!(f, "movzb %al, %rax");
        }
        | NodeKind::ND_LT => {
            writeln!(f, "cmp %rdi, %rax");
            writeln!(f, "setl %al");
            writeln!(f, "movzb %al, %rax");
        }
        | NodeKind::ND_BE => {
            writeln!(f, "cmp %rax, %rdi");
            writeln!(f, "setle %al");
            writeln!(f, "movzb %al, %rax");
        }
        | NodeKind::ND_BT => {
            writeln!(f, "cmp %rax, %rdi");
            writeln!(f, "setl %al");
            writeln!(f, "movzb %al, %rax");
        }
        | _ => {}
    }

    writeln!(f, "push %rax");
}

fn create_file(path: &str) -> File {
    let f = match File::create(path) {
        | Ok(f) => f,
        | Err(e) => panic!("{}", e),
    };
    return f;
}
