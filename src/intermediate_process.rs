use std::collections::HashMap;

use crate::{
    parse::{Function, Node, NodeKind},
    tokenize::Type,
};

// IdentID is a unique label for Functino's local variable,
// and generated from blockstr. This label holds variable's
// scope information.
pub type IdentID = String;

pub const FN_ARG_BLOC_STR: &str = "_1";

#[derive(Clone)]
pub struct Variable {
    pub typ: Type,
    pub offset: usize,
}
impl Variable {
    fn new(offset: usize, typ: Type) -> Self {
        return Variable {
            offset: offset,
            typ: typ,
        };
    }
}

#[derive(Clone)]
pub struct FunctionLocalVariable {
    // variable table hashmap which holds ident_id - val_offset
    pub val_table: HashMap<IdentID, Variable>,
    // current offset address from rbp.
    // this value will be updated each time ident-node is found.
    current_offset: usize,
}
impl FunctionLocalVariable {
    pub fn new() -> Self {
        return Self {
            val_table: HashMap::new(),
            current_offset: 0,
        };
    }
    // block_strとsymbolから、idnet_idを作成する.
    // ident_idがすでにident_id_mapに存在していたら(つまり同じscopeにおいて同じシンボルが定義されていたら)、
    // Errを返す.
    pub fn try_new_val_offset(
        &mut self,
        symbol: Symbol,
        typ: Type,
        blcstr: BlockStr,
    ) -> Result<Variable, &str> {
        let ident_id = blockstr_to_identid(symbol.clone(), blcstr.clone());
        match self.get_val_offset_by_identid(ident_id.clone()) {
            // すでに同じsymbolが同じscope内で宣言されている.
            | Some(_) => Err("Already Exist Symbol"),
            | None => {
                self.current_offset += 8;
                let v = Variable::new(self.current_offset, typ);
                self.val_table.insert(ident_id.clone(), v.clone());
                return Ok(v);
            }
        }
    }
    pub fn get_val_offset_by_identid(&self, ident_id: IdentID) -> Option<&Variable> {
        return self.val_table.get(&ident_id);
    }
    // 変数のsymbolとblcstrを受け取り、その変数のrbpからのoffsetを返す.
    // 同じblock内に検索しているsymbolがなかった場合、より浅いblockでの検索を
    // 繰り返す.最も浅いblock(関数のblock)にも該当するsymbolがなかった場合は
    // Noneを返す.
    pub fn get_val_offset_by_identid_recursively(
        &self,
        ident_id: IdentID,
    ) -> Option<Variable> {
        let depth = identid_to_depth(&ident_id);

        let mut current_ident_id = ident_id.clone();
        for _ in 0..depth {
            if let Some(val) = self.val_table.get(&current_ident_id) {
                return Some(val.clone());
            }
            // currentのdepthにない場合、current_ident_idを更新
            match upper_block_ident_id(&current_ident_id) {
                | Some(id) => current_ident_id = id,
                | None => (),
            };
        }
        // 該当するsymbolがなかった際.
        return None;
    }
}
// build identid from symbol, blockstr
pub fn blockstr_to_identid(symbol: Symbol, block_str: BlockStr) -> IdentID {
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
        | None => return None,
        | Some(_) => (),
    };
    return Some(v.join("_"));
}

// A structure that summarizes the information that is passed
// to the read_node().
//
// Currently, this struct is used mainly for labeling block-node
// to specify variable scopes.
struct ReadNodeArgs {
    // current block-node index at same depth.
    index: Vec<usize>,
    // current block-node depth.
    depth: usize,
    // hold current blc_str.
    cur_block_str: String,
    // current function's all variables. See IdentDir-struct part.
    local_variable: FunctionLocalVariable,
    // current size to which rsp lower when called this function.
    val_size: usize,
}
impl ReadNodeArgs {
    fn new() -> Self {
        return ReadNodeArgs {
            index: vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // TODO: 暫定的な処置
            depth: 1,
            cur_block_str: String::from("_1"),
            local_variable: FunctionLocalVariable::new(),
            val_size: 0,
        };
    }
}

// BlockStr is kind like hash value which should be unique in
// the same block-node depth. This hash value is determined by depth and index.
// Actual block_str example is below.
//
// [example]
//
// main() {
//  // here, block_str is `_1` (depth: 1)
//
//     {
//         // here, block_str is `_1_1` (index:1, depth:2)
//         {
//             // here, block_str is `_1_1_1` (index:1, depth:3)
//         }
//         {
//             // here, block_str is `_1_1_2` (index:2, depth:3)
//             {
//
//             }
//             {
//                 // here, block_str is `_1_1_2_2` (index:2, depth:4)
//             }
//         }
//     }
//     {
//         // here, block_str is `_1_2` (index:1, depth:2)
//     }
// }
pub type BlockStr = String;

// build blockStr from current depth and index.
pub fn build_block_str(depth: usize, index: &Vec<usize>) -> String {
    let mut base = String::from("");
    for i in 1..=depth {
        base.push_str(format!("_{}", index[i]).as_str())
    }
    return base;
}

// symbol is variable's symbol
pub type Symbol = String;

// Functionに対して、
// ·各block-nodeごとに変数をlabel(BlockStr)付けする.
// ·Function内のlocal変数の合計サイズを計算(関数呼び出し時に引き下げるrspの値の計算に使用).
//
// MEMO: (将来的には)最適化的なことを行う.
pub fn intermediate_process(fvec: Vec<Function>) -> Vec<Function> {
    let mut fvec_after_processed = vec![];

    for f in fvec.iter() {
        let mut f_clone = f.clone();

        // 関数の引数、およびのローカル変数をlocal_variableに格納.
        // identのtypeを辻褄合わせ.
        set_block_str_and_create_localval_table(&mut f_clone);

        fvec_after_processed.push(f_clone);
    }
    return fvec_after_processed;
}

// Read the all nodes owned by Function and create variable table.
// In addition, it counts size to which rsp lowered when called this function.
fn set_block_str_and_create_localval_table(f: &mut Function) {
    let mut nodes = f.root_node.fn_blocks.clone();
    let mut arg = ReadNodeArgs::new();

    for node in nodes.as_mut() as &mut Vec<Node> {
        read_node(node, &mut arg);
    }
    let root_node = Node {
        kind: NodeKind::ND_BLOCK,
        fn_blocks: nodes,
        ..Default::default()
    };

    // 関数の引数をlocal_variableに詰める
    let mut local_variable = arg.local_variable.clone();
    for (_, arg) in f.fn_args.iter().cloned().enumerate() {
        // TODO: 関数の引数はdepth, index共に0とする(後にきちんと仕様としてどこかにまとめる)
        let block_str = String::from(FN_ARG_BLOC_STR);
        let _ = local_variable
            .try_new_val_offset(arg.sym.clone(), Type::Unknown, block_str)
            .unwrap_or_else(|e| {
                panic!(
                    "Err: {}: Maybe symbol {} is duplicated in this function.",
                    e, arg.sym
                )
            });
    }

    f.root_node = root_node;
    f.lv_size = arg.val_size;
    f.local_variable = local_variable;
    return;
}

fn read_node(node: &mut Node, arg: &mut ReadNodeArgs) {
    /*
       idnet node.
    */
    if node.kind == NodeKind::ND_IDENT {
        // そのidentが作成されたblockを示す、block_strを入れる.
        // TODO: 多分使わなくなる
        node.block_str = arg.cur_block_str.clone();

        let ident_id = blockstr_to_identid(node.str.clone(), arg.cur_block_str.clone());
        if let Some(val) = arg
            .local_variable
            .get_val_offset_by_identid_recursively(ident_id.clone())
        {
            node.typ = val.typ
        }

        // TODO: 型を導入する時に改善/
        arg.val_size += 8;

        return;
    }
    if node.kind == NodeKind::ND_BLOCK {
        arg.depth += 1;
        arg.index[arg.depth] += 1;
        arg.cur_block_str = build_block_str(arg.depth, &arg.index);

        for block_stmt in node.block_stmts.as_mut() as &mut Vec<Node> {
            read_node(block_stmt, arg);
        }

        // depth以下の情報は破棄する.
        for i in (arg.depth + 1)..10 {
            arg.index[i] = 0;
        }
        arg.depth -= 1;
        arg.cur_block_str = build_block_str(arg.depth, &arg.index);
        return;
    }
    if node.kind == NodeKind::ND_NUM {
        return;
    }
    if node.kind == NodeKind::ND_PTR_REF {
        read_node(&mut node.ptr_ref_ident.as_mut().unwrap(), arg);
        return;
    }
    if node.kind == NodeKind::ND_PTR_DEREF {
        read_node(&mut node.ptr_deref_ident.as_mut().unwrap(), arg);
        return;
    }

    /*
       nodes that have next node in left side.
    */
    if node.kind == NodeKind::ND_EXPR
        || node.kind == NodeKind::ND_STMT
        || node.kind == NodeKind::ND_RETURN
        || node.kind == NodeKind::ND_IFCOND
        || node.kind == NodeKind::ND_ELSE
        || node.kind == NodeKind::ND_STMT2
    {
        read_node(&mut node.l.as_mut().unwrap(), arg);
        return;
    }

    /*
       irregular nodes that don't have next node neither in left nor right.
    */
    // for for_stmt
    if node.kind == NodeKind::ND_FOR {
        read_node(&mut node.for_node_first_assign.as_mut().unwrap(), arg);
        read_node(&mut node.for_node_second_condition.as_mut().unwrap(), arg);
        read_node(&mut node.for_node_third_expr.as_mut().unwrap(), arg);
        read_node(&mut node.for_node_stmts.as_mut().unwrap(), arg);
        return;
    }
    // for if_stmt
    if node.kind == NodeKind::ND_IFSTMT {
        read_node(&mut node.if_node.as_mut().unwrap(), arg);
        if node.elsif_node.is_some() {
            read_node(&mut node.elsif_node.as_mut().unwrap(), arg);
        }
        if node.else_node.is_some() {
            read_node(&mut node.else_node.as_mut().unwrap(), arg);
        }
        return;
    }
    // for fncall
    if node.kind == NodeKind::ND_FNCALL {
        for v in &mut node.fn_call_args {
            // 引数のそれぞれのNodeを展開する(ここでblock_strも付与される)
            read_node(&mut v.val.as_mut().unwrap(), arg)
        }
        // TODO: ここでTypeを詰める
        return;
    }

    /*
        read binary_node.
    */
    if node.kind == NodeKind::ND_ASSIGN {
        let block_str = build_block_str(arg.depth, &arg.index);
        let ident_id =
            blockstr_to_identid(node.l.as_ref().unwrap().str.clone(), block_str);
        // Err checkのため
        let _ = arg
            .local_variable
            .get_val_offset_by_identid_recursively(ident_id)
            .unwrap_or_else(|| {
                println!("variable not found. {}", node.l.as_ref().unwrap().str);
                panic!("");
            });

        read_node(&mut node.l.as_mut().unwrap(), arg);
        read_node(&mut node.r.as_mut().unwrap(), arg);
        return;
    }

    if node.kind == NodeKind::ND_DECL {
        let block_str = build_block_str(arg.depth, &arg.index);
        // TODO: declnにblockstrがひっついている構造
        node.block_str = block_str.clone();
        // Err checkのため
        let _ = match arg.local_variable.try_new_val_offset(
            node.l.as_ref().unwrap().str.clone(),
            Type::Unknown,
            block_str,
        ) {
            | Ok(v) => v,
            | Err(_) => {
                panic!("Symbol duplicated.")
            }
        };
        read_node(&mut node.l.as_mut().unwrap(), arg);
        read_node(&mut node.r.as_mut().unwrap(), arg);

        return;
    }
    read_node(&mut node.l.as_mut().unwrap(), arg);
    read_node(&mut node.r.as_mut().unwrap(), arg);
    return;
}
