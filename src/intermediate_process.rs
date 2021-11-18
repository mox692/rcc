use std::collections::HashMap;

use crate::parse::Function;
use crate::parse::Node;
use crate::parse::NodeKind;

struct FunctionVariableTale {
    pub tbl: Vec<String>,
}
impl FunctionVariableTale {
    fn new() -> Self {
        return Self { tbl: Vec::new() };
    }
    fn or_push(&mut self, str: String) -> bool {
        if self.search(str.clone()) {
            return false;
        }
        self.tbl.push(str.clone());
        return true;
    }
    fn search(&self, str: String) -> bool {
        for val in self.tbl.iter() {
            if val.as_str() == str.as_str() {
                return true;
            }
        }
        return false;
    }
}

// Functionに対して、local変数のカウントを行ったり、
// (将来的には)最適化的なことを行う.
pub fn intermediate_process(mut fvec: Vec<Function>) -> Vec<Function> {
    let mut fvec_after = vec![];
    for f in fvec.iter() {
        let mut f_clone = f.clone();
        // 関数のstack_sizeを計測.
        set_lvsize_to_function(&mut f_clone);
        // 関数のlocal変数表の作成.
        f_clone = set_block_str_and_create_localval_table(f_clone);
        fvec_after.push(f_clone);
    }
    return fvec_after;
}

fn set_lvsize_to_function(f: &mut Function) {
    let f_clone = f.clone();
    f.lv_size = count_fn_localval_size(f_clone);
}

fn count_fn_localval_size(f: Function) -> usize {
    let mut size: usize = 0;
    let mut val_tbl = FunctionVariableTale::new();
    for node in f.root_node.fn_blocks {
        let mut s: usize = 0;
        size += count_node_localval_size(&node, &mut val_tbl, &mut s)
    }
    return size;
}

fn count_node_localval_size(
    node: &Node,
    val_tbl: &mut FunctionVariableTale,
    size: &mut usize,
) -> usize {
    //
    // Terminal symbol.
    //
    if node.kind == NodeKind::ND_IDENT {
        if val_tbl.or_push(node.str.clone()) {
            *size += 1;
        }
        return *size;
    }
    if node.kind == NodeKind::ND_NUM {
        return *size;
    }

    // for ND_EXPR, ND_STMT.
    if node.kind == NodeKind::ND_EXPR
        || node.kind == NodeKind::ND_STMT
        || node.kind == NodeKind::ND_RETURN
    {
        count_node_localval_size(node.l.as_ref().unwrap(), val_tbl, size);
        return *size;
    }

    // for for_stmt
    if node.kind == NodeKind::ND_FOR {
        count_node_localval_size(node.for_node_first_assign.as_ref().unwrap(), val_tbl, size);
        count_node_localval_size(
            node.for_node_second_condition.as_ref().unwrap(),
            val_tbl,
            size,
        );
        count_node_localval_size(node.for_node_third_expr.as_ref().unwrap(), val_tbl, size);
        count_node_localval_size(node.for_node_stmts.as_ref().unwrap(), val_tbl, size);
        return *size;
    }

    // for if_stmt
    if node.kind == NodeKind::ND_IFSTMT {
        count_node_localval_size(node.if_node.as_ref().unwrap(), val_tbl, size);
        if node.elsif_node.is_some() {
            count_node_localval_size(node.elsif_node.as_ref().unwrap(), val_tbl, size);
        }
        if node.else_node.is_some() {
            count_node_localval_size(node.else_node.as_ref().unwrap(), val_tbl, size);
        }
        return *size;
    }

    if node.kind == NodeKind::ND_IFCOND {
        count_node_localval_size(node.l.as_ref().unwrap(), val_tbl, size);
        return *size;
    }

    if node.kind == NodeKind::ND_ELSE {
        count_node_localval_size(node.l.as_ref().unwrap(), val_tbl, size);
        return *size;
    }

    if node.kind == NodeKind::ND_BLOCK {
        let mut i = 0;
        loop {
            if i == node.block_stmts_len {
                break;
            }
            count_node_localval_size(&node.block_stmts[i], val_tbl, size);
            i += 1;
        }
        return *size;
    }

    if node.kind == NodeKind::ND_STMT2 {
        count_node_localval_size(node.l.as_ref().unwrap(), val_tbl, size);
        return *size;
    }

    if node.kind == NodeKind::ND_FNCALL {
        return *size;
    }

    /*
        read binary_node.
    */
    count_node_localval_size(node.l.as_ref().unwrap(), val_tbl, size);
    count_node_localval_size(node.r.as_ref().unwrap(), val_tbl, size);
    return *size;
}

// Functionが保有するnodeを読み、block_nodeに対してラベル付け(block_str)を行う.
// ラベル付けされたnodeを保有するFunctionを返す.
fn set_block_str_and_create_localval_table(f: Function) -> Function {
    let mut nodes = f.root_node.fn_blocks;
    let mut arg = ReadNodeArgs::new();
    let mut i = 0;
    loop {
        let mut node = nodes[i].clone();
        read_node(&mut node, &mut arg);
        // 書き換わったnodeを、元のnodeのvectorに書き戻す.
        nodes[i] = node;
        i += 1;
        if i == nodes.len() {
            for (k, v) in &*arg.ident_dir.dir {
                for (kk, vv) in &*v {
                    // for debug.
                    // println!("depth: {}, ident_name: {}, block_str: {}", k, vv, kk);
                }
            }
            break;
        }
    }
    let root_node = Node {
        kind: NodeKind::ND_BLOCK,
        fn_blocks: nodes,
        ..Default::default()
    };
    return Function {
        lv_size: f.lv_size,
        root_node: root_node,
    };
}

struct ReadNodeArgs {
    index: Vec<usize>,
    depth: usize,
    cur_str: String,
    // <usize(blockの階層), Hashmap(ident table)>
    ident_dir: IdentDir,
}
impl ReadNodeArgs {
    fn new() -> Self {
        return ReadNodeArgs {
            index: vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // TODO: 暫定的な処置
            depth: 1,
            cur_str: String::from("_1"),
            ident_dir: IdentDir::new(),
        };
    }
}

// Funcion内のsymbolを格納するstruct.
pub struct IdentDir {
    pub dir: Box<HashMap<usize, HashMap<String, String>>>,
}
impl IdentDir {
    pub fn new() -> Self {
        return Self {
            dir: Box::new(HashMap::new()),
        };
    }
    pub fn insert_nth_depth_identtable(&mut self, n: usize, ident_table: (String, String)) {
        let mut nth_ident_table = self.get_nth_depth_identtable_or(n);
        nth_ident_table.insert(ident_table.0, ident_table.1);
        self.dir.insert(n, nth_ident_table);
    }
    pub fn get_nth_depth_identtable_or(&self, n: usize) -> HashMap<String, String> {
        let nth_ident_table = match self.dir.get(&n) {
            Some(t) => t.clone(),
            _ => HashMap::new(),
        };
        return nth_ident_table;
    }
}

fn read_node(node: &mut Node, arg: &mut ReadNodeArgs) {
    //
    // Terminal symbol.
    //
    if node.kind == NodeKind::ND_IDENT {
        // MEMO: そのidentが作成されたblockを示す、block_strを入れる.
        node.block_str = arg.cur_str.clone();

        // TODO: ここのident tableにblock_strを入れる処理、method化したい.(ここの位置にあると見辛い)

        // // intmapのcloneを作成
        // let mut ident_table = match arg.ident_dir.get(&arg.depth) {
        //     Some(ident_table) => ident_table.clone(),
        //     // このdepthでの初めてのident_table entryを作る時
        //     None => HashMap::new(),
        // };
        // // cloneにこのidentnodeの識別子の情報を入れる.
        // ident_table.insert(node.block_str.clone(), String::from(node.str.clone()));

        arg.ident_dir.insert_nth_depth_identtable(
            arg.depth,
            (node.block_str.clone(), String::from(node.str.clone())),
        );
        return;
    }
    if node.kind == NodeKind::ND_BLOCK {
        let mut i = 0;
        // MEMO:
        /*  main() {
            // ここのblockは `_1` (depth: 1)

                {
                    // ここのblockは `_1_1` (index:1, depth:2)
                    {
                        // ここのblockは `_1_1_1` (index:1, depth:3)
                    }
                    {
                        // ここのblockは `_1_1_2` (index:2, depth:3)
                        {

                        }
                        {
                            // ここのblockは `_1_1_2_2` (index:2, depth:4)
                        }
                    }
                }
                {
                    // ここのblockは `_1_2` (index:1, depth:2)
                }
            }
        */
        // depth = 1
        arg.depth += 1;
        // index[1] = 1
        arg.index[arg.depth] += 1;
        arg.cur_str = block_str_from_index(arg.depth.clone(), &arg.index);
        loop {
            if i == node.block_stmts_len {
                break;
            }
            read_node(&mut node.block_stmts[i], arg);
            i += 1;
        }
        // depth以下の情報は破棄する.
        for i in (arg.depth + 1)..10 {
            arg.index[i] = 0;
        }
        arg.depth -= 1;
        arg.cur_str = block_str_from_index(arg.depth.clone(), &arg.index);
        return;
    }
    if node.kind == NodeKind::ND_NUM {
        return;
    }

    // for ND_EXPR, ND_STMT.
    if node.kind == NodeKind::ND_EXPR
        || node.kind == NodeKind::ND_STMT
        || node.kind == NodeKind::ND_RETURN
    {
        read_node(&mut node.l.as_mut().unwrap(), arg);
        return;
    }

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

    if node.kind == NodeKind::ND_IFCOND {
        read_node(&mut node.l.as_mut().unwrap(), arg);
        return;
    }

    if node.kind == NodeKind::ND_ELSE {
        read_node(&mut node.l.as_mut().unwrap(), arg);
        return;
    }

    if node.kind == NodeKind::ND_STMT2 {
        read_node(&mut node.l.as_mut().unwrap(), arg);
        return;
    }

    if node.kind == NodeKind::ND_FNCALL {
        return;
    }

    /*
        read binary_node.
    */
    read_node(&mut node.l.as_mut().unwrap(), arg);
    read_node(&mut node.r.as_mut().unwrap(), arg);
    return;
}

// depth = 1, index[1] = 1
fn block_str_from_index(depth: usize, index: &Vec<usize>) -> String {
    let mut base = String::from("");
    for i in 1..=depth {
        base.push_str(format!("_{}", index[i]).as_str())
    }
    return base;
}
