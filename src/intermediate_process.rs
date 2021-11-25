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
pub fn intermediate_process(fvec: Vec<Function>) -> Vec<Function> {
    let mut fvec_after_processed = vec![];
    for f in fvec.iter() {
        let mut f_clone = f.clone();

        // 関数のlocal変数表の作成.
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
    f.root_node = root_node;
    f.lv_size = arg.val_size;
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
    cur_str: String,
    // current function's all variables. See IdentDir-struct part.
    ident_dir: IdentDir,
    // current size to which rsp lower when called this function.
    val_size: usize,
}
impl ReadNodeArgs {
    fn new() -> Self {
        return ReadNodeArgs {
            index: vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // TODO: 暫定的な処置
            depth: 1,
            cur_str: String::from("_1"),
            ident_dir: IdentDir::new(),
            val_size: 0,
        };
    }
}

// IdentDir store all variable in current function.
// For each block-node depth, it holds hashmap of ident table.
pub struct IdentDir {
    pub dir: Box<HashMap<usize, IdentTable>>
}

type IdentTable = HashMap<BlockStr, Symbol>;

// BlockStr is kind like hash value which should be unique in 
// the same block-node depth. This hash value is determined by depth and index.
// Actual block_str example is below.
//
// [example]
// 
/*  
    main() {
    // here, block_str is `_1` (depth: 1)

        {
            // here, block_str is `_1_1` (index:1, depth:2)
            {
                // here, block_str is `_1_1_1` (index:1, depth:3)
            }
            {
                // here, block_str is `_1_1_2` (index:2, depth:3)
                {

                }
                {
                    // here, block_str is `_1_1_2_2` (index:2, depth:4)
                }
            }
        }
        {
            // here, block_str is `_1_2` (index:1, depth:2)
        }
    }
*/
type BlockStr = String;

// symbol is variable's symbol
type Symbol = String; 

impl IdentDir {
    pub fn new() -> Self {
        return Self {
            dir: Box::new(HashMap::new()),
        };
    }
    // register ident_table entry to nth deph IdentTable.
    pub fn insert_nth_depth_identtable(&mut self, n: usize, ident_table_ent: (BlockStr, Symbol)) {
        let mut nth_ident_table = self.get_nth_depth_identtable_or(n);
        nth_ident_table.insert(ident_table_ent.0, ident_table_ent.1);
        self.dir.insert(n, nth_ident_table);
    }
    // get ident_table entry from nth deph IdentTable.
    pub fn get_nth_depth_identtable_or(&self, n: usize) -> HashMap<BlockStr, Symbol> {
        let nth_ident_table = match self.dir.get(&n) {
            Some(t) => t.clone(),
            _ => HashMap::new(),
        };
        return nth_ident_table;
    }
}

fn read_node(node: &mut Node, arg: &mut ReadNodeArgs) {
    /*
        idnet node.
     */
    if node.kind == NodeKind::ND_IDENT {
        // そのidentが作成されたblockを示す、block_strを入れる.
        node.block_str = arg.cur_str.clone();
        arg.ident_dir.insert_nth_depth_identtable(
            arg.depth,
            (node.block_str.clone(), String::from(node.str.clone())),
        );

        // increment arg.size according to its variable type.
        arg.val_size += 1;
        
        return;
    }

    /*
        block node.
     */
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
    if node.kind == NodeKind::ND_BLOCK {
        let mut i = 0;
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
