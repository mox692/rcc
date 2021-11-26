use crate::intermediate_process::BlockStr;
use crate::intermediate_process::Symbol;
use crate::parse::Function;
use crate::parse::Node;
use crate::parse::NodeKind;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

// IdentID is a unique label for Functino's local variable,
// and generated from blockstr. This label holds variable's
// scope information.
type IdentID = String;

// build identid from symbol, blockstr
fn blockstr_to_identid(symbol: Symbol, block_str: BlockStr) -> IdentID {
    return format!("{}{}", symbol, block_str);
}
fn identid_to_depth(ident_id: &IdentID) -> usize {
    let mut count = 0;
    for c in ident_id.chars() {
        if c.eq(&'_') {
            count += 1;
        }
    }
    return count;
}
// ident_idを受け取り、その1つ上のblockにある同名symbolを表すblockstrを返す.
// ex:
// _1_2 => _1
// _1_2_3 => _1_2
// _1 => None
fn upper_block_ident_id(ident_id: &String) -> Option<String> {
    let mut v: Vec<&str> = ident_id.split("_").collect();
    match v.pop() {
        None => return None,
        Some(_) => {}
    };
    return Some(v.join("_"));
}

struct FunctionLocalVariable {
    // variable table hashmap which holds ident_id - val_offset
    val_table: HashMap<IdentID, usize>,
    // current offset address from rbp.
    // this value will be updated each time ident-node is found.
    current_offset: usize,
}
impl FunctionLocalVariable {
    fn new() -> Self {
        return Self {
            val_table: HashMap::new(),
            current_offset: 0,
        };
    }
    // block_strとsymbolから、idnet_idを作成する.
    // ident_idがすでにident_id_mapに存在していたら(つまり同じscopeにおいて同じシンボルが定義されていたら)、
    // Errを返す.
    fn try_new_val_offset(&mut self, symbol: Symbol, blcstr: BlockStr) -> Result<usize, &str> {
        let ident_id = blockstr_to_identid(symbol, blcstr.clone());
        match self.get_val_offset_by_identid(ident_id.clone()) {
            // すでに同じsymbolが同じscope内で宣言されている.
            Some(_) => Err("Already Exist Symbol"),
            None => {
                self.current_offset += 8;
                self.val_table.insert(ident_id.clone(), self.current_offset);
                return Ok(self.current_offset);
            }
        }
    }
    fn get_val_offset_by_identid(&self, ident_id: IdentID) -> Option<&usize> {
        return self.val_table.get(&ident_id);
    }
    // 変数のsymbolとblcstrを受け取り、その変数のrbpからのoffsetを返す.
    // 同じblock内に検索しているsymbolがなかった場合、より浅いblockでの検索を
    // 繰り返す.最も浅いblock(関数のblock)にも該当するsymbolがなかった場合は
    // Noneを返す.
    fn get_val_offset_by_identid_recursively(&self, ident_id: IdentID) -> Option<usize> {
        let depth = identid_to_depth(&ident_id);

        let mut current_ident_id = ident_id.clone();
        for _ in 0..=depth - 1 {
            match self.val_table.get(&current_ident_id) {
                Some(offset) => return Some(offset.clone()),
                None => {}
            }
            // currentのdepthにない場合、current_ident_idを更新
            match upper_block_ident_id(&current_ident_id) {
                Some(id) => current_ident_id = id,
                None => {}
            };
        }
        // 該当するsymbolがなかった際.
        return None;
    }
}

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
    fn increment(&mut self) {
        self.cur_index += 1;
    }
}

pub fn codegen(functions: Vec<Function>) {
    for f in functions.iter() {
        codegen_func(f.clone());
    }
}

pub fn codegen_func(function: Function) {
    let root_node = &function.root_node;

    // let mut lv = LocalVariable::new();
    let mut lv = FunctionLocalVariable::new();
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
    for node in root_node.fn_blocks.clone() {
        gen(&node, &mut f, &mut lv, &mut cl);
    }

    writeln!(f, "pop %rax");
    writeln!(f, "mov %rbp, %rsp");
    writeln!(f, "pop %rbp");
    writeln!(f, "ret");
}

// 引数で渡されたNodeを展開して、その評価結果をstack topにpushする.
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
        let offset = lv
            .get_val_offset_by_identid_recursively(blockstr_to_identid(
                node.str.clone(),
                node.block_str.clone(),
            ))
            .unwrap_or_else(|| panic!("Undeclared symbol!!"));
        writeln!(f, "lea -{}(%rbp), %rax", offset);
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
        // 右辺のoffsetをs取得
        let offset = lv
            .get_val_offset_by_identid_recursively(blockstr_to_identid(
                node.l.as_ref().unwrap().str.clone(),
                node.l.as_ref().unwrap().block_str.clone(),
            ))
            .unwrap_or_else(|| panic!("Undecleare symbol!!"));
        // 左辺のstrと紐付けた形でstack上にデータ領域を確保.
        // -> ND_EXPRのcodeを生成.
        writeln!(f, "lea -{}(%rbp), %rax", offset);
        writeln!(f, "push %rax");

        gen(node.r.as_ref().unwrap().as_ref(), f, lv, cl);
        writeln!(f, "pop %rax");
        writeln!(f, "pop %rdi");
        writeln!(f, "mov %rax, (%rdi)");
        return;
    }
    if node.kind == NodeKind::ND_BLOCK {
        let node_vec = node.block_stmts.clone();
        let len = node.block_stmts_len;
        let mut i = 0;
        loop {
            gen(&node_vec[i], f, lv, cl);
            i += 1;
            if len == i {
                break;
            }
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

    if node.kind == NodeKind::ND_DECL {
        let offset = match lv.try_new_val_offset(
            node.l.as_ref().unwrap().str.clone(),
            node.l.as_ref().unwrap().block_str.clone(),
        ) {
            Ok(v) => v,
            Err(_) => panic!("Symbol duplicated!!"),
        };

        writeln!(f, "lea -{}(%rbp), %rax", offset);
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
