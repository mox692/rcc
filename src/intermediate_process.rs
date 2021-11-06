use std::ops::Index;

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
pub fn intermediate_process(mut f: Function) -> Function {
    set_lvsize_to_function(&mut f);
    set_block_str_and_create_localval_table(&mut f);
    return f;
}

fn set_lvsize_to_function(f: &mut Function) {
    let f_clone = f.clone();
    f.lv_size = count_fn_localval_size(f_clone);
}

fn count_fn_localval_size(f: Function) -> usize {
    let mut size: usize = 0;
    let mut val_tbl = FunctionVariableTale::new();
    for node in f.nodes {
        let mut s: usize = 0;
        size += count_node_localval_size(node.as_ref(), &mut val_tbl, &mut s)
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
            count_node_localval_size(node.block_stmts[i].as_ref().unwrap(), val_tbl, size);
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

fn set_block_str_and_create_localval_table(f: &mut Function) {
    let mut index: Vec<usize> = vec![0; 10];
    let mut depth: usize = 0;
    let mut cur_str = String::new();
    for node in f.nodes.clone() {
        read_node(node.as_ref(), &mut index, &mut depth, &mut cur_str)
    }
    return;
}

fn read_node(node: &Node, index: &mut Vec<usize>, depth: &mut usize, cur_str: &mut String) {
    //
    // Terminal symbol.
    //
    if node.kind == NodeKind::ND_IDENT {
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
        read_node(node.l.as_ref().unwrap(), index, depth, cur_str);
        return;
    }

    // for for_stmt
    if node.kind == NodeKind::ND_FOR {
        read_node(
            node.for_node_first_assign.as_ref().unwrap(),
            index,
            depth,
            cur_str,
        );
        read_node(
            node.for_node_second_condition.as_ref().unwrap(),
            index,
            depth,
            cur_str,
        );
        read_node(
            node.for_node_third_expr.as_ref().unwrap(),
            index,
            depth,
            cur_str,
        );
        read_node(node.for_node_stmts.as_ref().unwrap(), index, depth, cur_str);
        return;
    }

    // for if_stmt
    if node.kind == NodeKind::ND_IFSTMT {
        read_node(node.if_node.as_ref().unwrap(), index, depth, cur_str);
        if node.elsif_node.is_some() {
            read_node(node.elsif_node.as_ref().unwrap(), index, depth, cur_str);
        }
        if node.else_node.is_some() {
            read_node(node.else_node.as_ref().unwrap(), index, depth, cur_str);
        }
        return;
    }

    if node.kind == NodeKind::ND_IFCOND {
        read_node(node.l.as_ref().unwrap(), index, depth, cur_str);
        return;
    }

    if node.kind == NodeKind::ND_ELSE {
        read_node(node.l.as_ref().unwrap(), index, depth, cur_str);
        return;
    }

    if node.kind == NodeKind::ND_BLOCK {
        let mut i = 0;
        // MEMO:
        /*  main() {
            // ここのblockは `_0`

                {
                    // ここのblockは `_1` (index:1, depth:1)
                    {
                        // ここのblockは `_1_1` (index:1, depth:2)
                    }
                    {
                        // ここのblockは `_1_2` (index:2, depth:2)
                        {

                        }
                        {
                            // ここのblockは `_1_2_2` (index:2, depth:3)
                        }
                    }
                }
                {
                    // ここのblockは `_2`
                }
            }
        */

        *depth += 1; // depthを1に
        index[*depth] += 1; // depth1のindexを1に
        let _ = block_str_from_index(depth.clone(), index);
        loop {
            if i == node.block_stmts_len {
                break;
            }
            read_node(node.block_stmts[i].as_ref().unwrap(), index, depth, cur_str);
            i += 1;
        }
        // depth以下の情報は破棄する.
        for i in (*depth + 1)..10 {
            index[i] = 0;
        }
        *depth -= 1;
        return;
    }

    if node.kind == NodeKind::ND_STMT2 {
        read_node(node.l.as_ref().unwrap(), index, depth, cur_str);
        return;
    }

    if node.kind == NodeKind::ND_FNCALL {
        return;
    }

    /*
        read binary_node.
    */
    read_node(node.l.as_ref().unwrap(), index, depth, cur_str);
    read_node(node.r.as_ref().unwrap(), index, depth, cur_str);
    return;
}

fn block_str_from_index(depth: usize, index: &Vec<usize>) -> String {
    let mut base = String::from("");
    for i in 1..=depth {
        base.push_str(format!("_{}", index[i]).as_str())
    }
    println!("block str: {}", base);
    return base;
}
