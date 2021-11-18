use crate::tokenize::{TokenKind, TokenReader, Type};

#[derive(Clone)]
pub struct Function {
    // Root Function Node
    pub root_node: Node,
    // The size at which the code generator lowers the stack.
    pub lv_size: usize,
}
impl Function {
    pub fn new(root_node: Node) -> Function {
        return Function {
            root_node: root_node,
            // TODO: calc lv from nodes.
            lv_size: 0,
        };
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub l: Option<Box<Node>>,
    pub r: Option<Box<Node>>,
    // for num node. (should be 0 in other node.)
    pub val: i32,
    // for ident node. (should be "" in other node.)
    pub str: String,

    // for if stmt
    pub if_node: Option<Box<Node>>,
    pub elsif_node: Option<Box<Node>>,
    pub else_node: Option<Box<Node>>,
    pub if_cond: Option<Box<Node>>,

    pub for_node: Option<Box<Node>>,
    pub for_node_first_assign: Option<Box<Node>>,
    pub for_node_second_condition: Option<Box<Node>>,
    pub for_node_third_expr: Option<Box<Node>>,
    pub for_node_stmts: Option<Box<Node>>,

    pub fn_name: String,

    // for block
    pub block_stmts: Vec<Node>,
    pub block_stmts_len: usize,

    // for creating block_str
    pub block_str: String,

    // 変数宣言nodeの
    pub decl_type: Type,

    // function
    pub fn_type: Type,
    pub fn_ident: String,
    pub fn_blocks: Vec<Node>, // BoxじゃないNode!!
}
impl Default for Node {
    fn default() -> Self {
        return Node {
            // `kind` must be override.
            kind: NodeKind::ND_NUM,
            l: None,
            r: None,
            val: 0,
            str: String::new(),
            if_node: None,
            elsif_node: None,
            else_node: None,
            if_cond: None,
            for_node: None,
            for_node_first_assign: None,
            for_node_second_condition: None,
            for_node_third_expr: None,
            for_node_stmts: None,
            block_stmts: Vec::new(),
            block_stmts_len: 0,
            fn_name: String::new(),
            block_str: String::new(),
            decl_type: Type::None,
            fn_type: Type::None,
            fn_ident: String::new(),
            fn_blocks: Vec::new(),
        };
    }
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NodeKind {
    ND_ROOT,
    ND_NUM,
    ND_ADD,
    ND_SUB,
    ND_MUL,
    ND_DIV,
    ND_EXPR,
    ND_ASSIGN,
    ND_IDENT,
    ND_STMT,
    ND_RETURN,
    ND_EQ,
    ND_NEQ,
    ND_BT,
    ND_BE,
    ND_LT,
    ND_LE,
    ND_IFSTMT,
    ND_IF,
    ND_ELSE,
    ND_ELSIF,
    ND_IFCOND,
    ND_FOR,
    ND_STMT2,
    ND_FNCALL,
    ND_BLOCK,
    ND_DECL,
}
fn gen_expr(expr_node: Option<Box<Node>>, tok: &mut TokenReader) -> Option<Box<Node>> {
    let node = Some(Box::new(Node {
        kind: NodeKind::ND_EXPR,
        l: expr_node,
        ..Default::default()
    }));
    return node;
}

fn gen_num_node(tok: &mut TokenReader) -> Option<Box<Node>> {
    if tok.cur_tok().kind.eq(&TokenKind::NUM) {
        let node = Some(Box::new(Node {
            kind: NodeKind::ND_NUM,
            val: tok.cur_tok().value,
            ..Default::default()
        }));
        tok.next();
        return node;
    }
    tok.error(String::from(format!(
        "expect num token, but got {:?}.",
        tok.cur_tok().kind
    )));
    panic!();
}

fn gen_stmt(tok: &mut TokenReader, node: Option<Box<Node>>) -> Option<Box<Node>> {
    let node = Some(Box::new(Node {
        kind: NodeKind::ND_STMT,
        l: node,
        ..Default::default()
    }));
    // MEMO: curを";"の次に指す
    tok.next();
    return node;
}

fn gen_ident_node(tok: &mut TokenReader) -> Option<Box<Node>> {
    let node = Some(Box::new(Node {
        kind: NodeKind::ND_IDENT,
        str: String::from(tok.cur_tok().char),
        ..Default::default()
    }));
    // MEMO: curを";"にして戻る.
    tok.next();
    return node;
}

fn gen_equality_node(
    nodekind: NodeKind,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
) -> Option<Box<Node>> {
    let node = Some(Box::new(Node {
        kind: nodekind,
        l: l,
        r: r,
        ..Default::default()
    }));
    return node;
}

fn gen_return_node(ret_stmt: Option<Box<Node>>) -> Option<Box<Node>> {
    let node = Some(Box::new(Node {
        kind: NodeKind::ND_RETURN,
        l: ret_stmt,
        ..Default::default()
    }));
    return node;
}

// num_node or ident_node.
fn gen_binary_node(kind: NodeKind, l: Option<Box<Node>>, r: Option<Box<Node>>) -> Node {
    return Node {
        kind: kind,
        l: l,
        r: r,
        ..Default::default()
    };
}

// gen if node. (l: if_cond, r: stmts )
fn gen_if_node(l: Option<Box<Node>>, r: Option<Box<Node>>) -> Option<Box<Node>> {
    return Some(Box::new(Node {
        kind: NodeKind::ND_IF,
        l: l,
        r: r,
        ..Default::default()
    }));
}

fn gen_fn_call_node(fn_name: String) -> Option<Box<Node>> {
    return Some(Box::new(Node {
        kind: NodeKind::ND_FNCALL,
        fn_name: fn_name,
        ..Default::default()
    }));
}

fn parse_fn_call(tok: &mut TokenReader, fn_name: String) -> Option<Box<Node>> {
    // TODO: 引数を読む処理.
    // ...
    // 今はfoo()という、引数なしの関数しか許していない.

    if tok.cur_tok().char != ")" {
        tok.error(String::from("expect `)`, but not."));
        panic!("");
    }
    // `)`の次のtokenを指すように.
    tok.next();
    return gen_fn_call_node(fn_name);
}

// unary = &num | &ident | &ident "(" ")"
fn parse_unary(tok: &mut TokenReader) -> Option<Box<Node>> {
    if tok.cur_tok().kind == TokenKind::NUM {
        return gen_num_node(tok);
    } else if tok.cur_tok().kind == TokenKind::IDENT {
        if tok.get_next_tok().char == "(" {
            // 呼び出し先で、`(`の次を読める様に.
            let fn_name = tok.cur_tok().char;
            return parse_fn_call(tok.next_nth_tok(2), fn_name);
        } else {
            return gen_ident_node(tok);
        }
    } else {
        tok.error(String::from("expect TokenKind::IDENT, but not."));
        panic!("");
    }
}

// mul_div = unary ( "*" unary | "/" unary )*
fn parse_mul_div(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut node = parse_unary(tok);
    loop {
        match tok.cur_tok().char.as_str() {
            "*" => {
                node = Some(Box::new(gen_binary_node(
                    NodeKind::ND_MUL, // TODO: ND_IDENTも対応.
                    node,
                    parse_unary(tok.next_tok()),
                )))
            }
            "/" => {
                node = Some(Box::new(gen_binary_node(
                    NodeKind::ND_DIV,
                    node,
                    parse_unary(tok.next_tok()),
                )))
            }
            _ => break,
        }
    }
    return node;
}

// generate ND_ADD or ND_SUB node.
// add_sub = mul_div("+" mul_div | "-" mul_div)*
fn parse_add_sub(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut node = parse_mul_div(tok);
    loop {
        match tok.cur_tok().char.as_str() {
            "+" => {
                node = Some(Box::new(gen_binary_node(
                    NodeKind::ND_ADD,
                    node,
                    parse_mul_div(tok.next_tok()),
                )))
            }
            "-" => {
                node = Some(Box::new(gen_binary_node(
                    NodeKind::ND_SUB,
                    node,
                    parse_mul_div(tok.next_tok()),
                )))
            }
            _ => break,
        };
    }

    return node;
}

// generate expression.
// expr = add_sub
fn parse_expr(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut node = parse_add_sub(tok);
    node = gen_expr(node, tok);
    return node;
}

// assign = &ident ( "=" equality )*
fn parse_assign(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut node = gen_ident_node(tok);
    if tok.expect("=") {
        node = Some(Box::new(gen_binary_node(
            NodeKind::ND_ASSIGN,
            node,
            parse_equality(tok.next_tok()),
        )));
    } else {
    }
    return node;
}

// l: equality, r: stmts
fn gen_elsif_node(l: Option<Box<Node>>, r: Option<Box<Node>>) -> Option<Box<Node>> {
    return Some(Box::new(Node {
        l: l,
        r: r,
        kind: NodeKind::ND_ELSIF,
        ..Default::default()
    }));
}

// elsif_node = "else if" "(" if_cond ")" stmts
fn parse_elsif(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut node: Option<Box<Node>>;
    if tok.cur_tok().char == "(" {
        node = parse_ifcond(tok.next_tok())
    } else {
        tok.error(String::from("parse if err."));
        panic!();
    }
    if tok.cur_tok().char == ")" {
        node = gen_elsif_node(node, parse_stmts2(tok.next_tok()));
    } else {
        tok.error(String::from("parse if err."));
        panic!();
    }
    return node;
}

fn gen_ifcond(node: Option<Box<Node>>) -> Option<Box<Node>> {
    return Some(Box::new(Node {
        kind: NodeKind::ND_IFCOND,
        l: node,
        ..Default::default()
    }));
}

// if_cond = equality
fn parse_ifcond(tok: &mut TokenReader) -> Option<Box<Node>> {
    return gen_ifcond(parse_equality(tok));
}

// if_node = "(" if_cond ")" stmts
fn parse_if(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut node: Option<Box<Node>>;
    if tok.cur_tok().char == "(" {
        node = parse_ifcond(tok.next_tok());
    } else {
        tok.error(String::from("parse if err."));
        panic!();
    }
    if tok.cur_tok().char == ")" {
        node = gen_if_node(node, parse_stmts2(tok.next_tok()));
    } else {
        tok.error(String::from("parse if err."));
        panic!();
    }
    return node;
}

fn gen_else(node: Option<Box<Node>>) -> Option<Box<Node>> {
    return Some(Box::new(Node {
        kind: NodeKind::ND_ELSE,
        l: node,
        ..Default::default()
    }));
}

// else_node = "else" stmts
fn parse_else(tok: &mut TokenReader) -> Option<Box<Node>> {
    let node = gen_else(parse_stmts2(tok));
    return node;
}

fn gen_ifstmt_node(
    if_node: Option<Box<Node>>,
    elsif_node: Option<Box<Node>>,
    else_node: Option<Box<Node>>,
) -> Option<Box<Node>> {
    return Some(Box::new(Node {
        kind: NodeKind::ND_IFSTMT,
        if_node: if_node,
        elsif_node: elsif_node,
        else_node: else_node,
        ..Default::default()
    }));
}

// ifstmt = "if" if_node ( elsif_node )? ( else_node )?
fn parse_ifstmt(tok: &mut TokenReader) -> Option<Box<Node>> {
    let if_node = parse_if(tok);
    let elif_node: Option<Box<Node>>;
    let else_node: Option<Box<Node>>;

    if tok.cur_tok().kind == TokenKind::ELIF {
        elif_node = parse_elsif(tok.next_tok());
        if tok.cur_tok().kind == TokenKind::ELSE {
            else_node = parse_else(tok.next_tok());
            return gen_ifstmt_node(if_node, elif_node, else_node);
        } else {
            return gen_ifstmt_node(if_node, elif_node, None);
        }
    } else {
        if tok.cur_tok().kind == TokenKind::ELSE {
            else_node = parse_else(tok.next_tok());
            return gen_ifstmt_node(if_node, None, else_node);
        } else {
            return gen_ifstmt_node(if_node, None, None);
        }
    }
}

// forstmt = "for" "(" declare ";" equality ";" expr ")" stmts
fn parse_forstmt(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut node: Box<Node> = Box::new(Node {
        kind: NodeKind::ND_FOR,
        ..Default::default()
    });
    if tok.cur_tok().char == "(" {
        // for(int a = 3;)

        let t = match tok.get_next_tok().kind {
            TokenKind::TYPE(t) => t,
            _ => panic!("fds"),
        };

        node.for_node_first_assign = parse_declare(tok.next_tok(), t);
    } else {
        tok.error(String::from("parse for err.(expect `(`)"));
        panic!();
    }
    if tok.cur_tok().char == ";" {
        node.for_node_second_condition = parse_equality(tok.next_tok());
    } else {
        tok.error(String::from("parse for err.(expect `;`)"));
        panic!();
    }
    if tok.cur_tok().char == ";" {
        node.for_node_third_expr = parse_expr(tok.next_tok());
    } else {
        tok.error(String::from("parse for err.(expect `;`)"));
        panic!();
    }

    if tok.expect(";") {
        tok.next_tok();
    } else {
        tok.error(String::from("parse for err.(expect `;`)"));
        panic!();
    }

    if tok.cur_tok().char == ")" {
        node.for_node_stmts = parse_stmts2(tok.next_tok());
    } else {
        tok.error(String::from("parse for err.(expect `)`"));
        panic!();
    }
    return Some(node);
}

// return = "return" equality
fn parse_return(tok: &mut TokenReader) -> Option<Box<Node>> {
    let node = gen_return_node(parse_equality(tok.next_tok()));
    return node;
}

// equality = expr ( "==" expr | "!=" expr | "<=" expr | ">=" expr | ">" expr | "<" expr )*
fn parse_equality(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut node = parse_expr(tok);
    if tok.cur_tok().kind == TokenKind::EQ {
        // TODO: nth_next的なものに置き換えたい.
        node = gen_equality_node(NodeKind::ND_EQ, node, parse_expr(tok.next_tok()));
    } else if tok.cur_tok().kind == TokenKind::NEQ {
        node = gen_equality_node(NodeKind::ND_NEQ, node, parse_expr(tok.next_tok()));
    } else if tok.cur_tok().kind == TokenKind::BE {
        node = gen_equality_node(NodeKind::ND_BE, node, parse_expr(tok.next_tok()));
    } else if tok.cur_tok().kind == TokenKind::BT {
        node = gen_equality_node(NodeKind::ND_BT, node, parse_expr(tok.next_tok()));
    } else if tok.cur_tok().kind == TokenKind::LT {
        node = gen_equality_node(NodeKind::ND_LT, node, parse_expr(tok.next_tok()));
    } else if tok.cur_tok().kind == TokenKind::LE {
        node = gen_equality_node(NodeKind::ND_LE, node, parse_expr(tok.next_tok()));
    }
    // MEMO: codegenの都合で、 ==, != を含まないexprは、equalityでwrapしないで、
    //       そのままexpr nodeとして返す.
    return node;
}

// declare = &type &ident "=" equality
// MEMO: typeより後ろはassign式と同じだが、コードジェネレータの都合で、
// declareの中にassignを入れるようなことはしない.
fn parse_declare(tok: &mut TokenReader, t: Type) -> Option<Box<Node>> {
    // cur -> &type
    let ident_node = gen_ident_node(tok.next_tok());
    if !tok.expect("=") {
        panic!("");
    }
    let equality_node = parse_equality(tok.next_tok());

    return Some(Box::new(Node {
        kind: NodeKind::ND_DECL,
        l: ident_node,
        r: equality_node,
        decl_type: t,
        ..Default::default()
    }));
}

// stmt = ( declare | assign | return | equality ) ";"
fn parse_stmt(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut node: Option<Box<Node>>;
    match tok.cur_tok().kind {
        TokenKind::RETURN => {
            node = parse_return(tok);
        }
        TokenKind::TYPE(t) => {
            node = parse_declare(tok, t);
        }
        // equality or assign
        TokenKind::IDENT => {
            if tok.get_next_tok().char == "=" {
                node = parse_assign(tok);
            } else {
                node = parse_equality(tok);
            }
        }
        _ => {
            node = parse_equality(tok);
        }
    };
    // if tok.cur_tok().kind == TokenKind::TYPE {
    //     node = parse_declare(tok);
    // } else if tok.cur_tok().kind == TokenKind::IDENT && tok.get_next_tok().char == "=" {
    //     node = parse_assign(tok);
    // } else if tok.cur_tok().kind == TokenKind::RETURN {
    //     node = parse_return(tok);
    // } else {
    //     node = parse_equality(tok);
    // }

    // MEMO: ここではcurは";"を指している.
    if tok.expect(";") {
        node = gen_stmt(tok, node);
        // MEMO: ここではcurは";"の次を指している.
        return node;
    }

    tok.error(String::from("expect ';', but not found."));
    panic!("");
}

// block = "{" stmts* "}"
fn parse_block(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut stmts: Vec<Node> = Vec::new();
    let mut node = Box::new(Node {
        kind: NodeKind::ND_BLOCK,
        ..Default::default()
    });
    let mut c = 0;
    loop {
        let _node = parse_stmts(tok).unwrap().as_ref().clone();
        stmts.push(_node);
        c += 1;
        if tok.cur_tok().char == "}" {
            node.block_stmts = stmts;
            node.block_stmts_len = c;
            tok.next();
            return Some(node);
        }
    }
}

// stmts2 = block | stmt
fn parse_stmts2(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut node = Box::new(Node {
        kind: NodeKind::ND_STMT2,
        ..Default::default()
    });
    if tok.cur_tok().char == "{" {
        let _node = parse_block(tok.next_tok());
        node.l = _node;
        return Some(node);
    }
    // one stmt.
    let _node = parse_stmt(tok);
    node.l = _node;
    return Some(node);
}

// stmts = ( stmts2 | ifstmt | forstmt )
fn parse_stmts(tok: &mut TokenReader) -> Option<Box<Node>> {
    let node: Option<Box<Node>>;
    if tok.cur_tok().kind == TokenKind::IF {
        node = parse_ifstmt(tok.next_tok());
        return node;
    }
    if tok.cur_tok().kind == TokenKind::FOR {
        node = parse_forstmt(tok.next_tok());
        return node;
    }
    node = parse_stmts2(tok);
    return node;
}

// function = int ident "(" ")" block
fn parse_function(tok: &mut TokenReader) -> Function {
    let t = match tok.cur_tok().kind {
        TokenKind::TYPE(t) => t,
        _ => panic!("expected type!!"),
    };

    let fn_ident = gen_ident_node(tok.next_tok());

    if tok.cur_tok().char == "(" && tok.get_next_tok().char == ")" {
        tok.next_nth_tok(3);
    } else {
        panic!("Invalid format.")
    }

    // MEMO: 純正のNodeを返すように.
    // MEMO: コード(Nodeが何もない時に、unwrap_or_elseがErrorになりそう.)
    let fn_block_nodes = parse_block(tok)
        .unwrap_or_else(|| panic!("No program input!!"))
        .as_ref()
        .clone();

    let n = Node {
        kind: NodeKind::ND_ROOT,
        fn_blocks: fn_block_nodes.block_stmts,
        ..Default::default()
    };
    let function = Function::new(n);
    return function;
}

// program = function
fn parse_program(tok: &mut TokenReader) -> Function {
    let f = parse_function(tok);
    return f;
}

// generate several nodes, and return Function.
// node = program
pub fn parse(tok: &mut TokenReader) -> Vec<Function> {
    // TODO: ini tok要る?
    consume_initial_tok(tok);
    let function = parse_program(tok);

    // TODO: 文法を変えて、複数のfunctionをparseできるように.
    //       今は手動でVec<Function>を返すようにしてる
    let fv = vec![function];
    return fv;
}

pub fn consume_initial_tok(tok: &mut TokenReader) {
    if tok.cur_tok().kind != TokenKind::INI {
        println!("expect INI TOKEN, but got {:?}", tok.cur_tok().kind);
        panic!("ERR");
    }
    tok.next();
}

pub fn debug_functions(flag: bool, functions: &Vec<Function>) {
    if !flag {
        return;
    }
    for function in functions.iter() {
        debug_nodes(&function.root_node.fn_blocks);
    }
}

pub fn debug_nodes(nodes: &Vec<Node>) {
    // TODO: fn loop
    println!("////////NODE DEBUG START////////");
    println!("{}'s nodes found.", nodes.len());
    for node in nodes.iter() {
        // 現段階では1つのstatement毎にdepthをresetする.
        let mut depth = 0;
        read_node(node, &mut depth);
    }
    println!("////////NODE DEBUG END////////");
}

pub fn read_node(node: &Node, depth: &mut usize) {
    print_node_info(node, depth);

    // for ND_NUM & ND_IDENT.
    if node.kind == NodeKind::ND_NUM || node.kind == NodeKind::ND_IDENT {
        return;
    }

    // for ND_EXPR, ND_STMT.
    if node.kind == NodeKind::ND_EXPR
        || node.kind == NodeKind::ND_STMT
        || node.kind == NodeKind::ND_RETURN
    {
        *depth += 1;
        read_node(node.l.as_ref().unwrap(), depth);
        *depth -= 1;
        return;
    }

    // for for_stmt
    if node.kind == NodeKind::ND_FOR {
        *depth += 1;
        read_node(node.for_node_first_assign.as_ref().unwrap(), depth);
        read_node(node.for_node_second_condition.as_ref().unwrap(), depth);
        read_node(node.for_node_third_expr.as_ref().unwrap(), depth);
        read_node(node.for_node_stmts.as_ref().unwrap(), depth);
        *depth -= 1;
        return;
    }

    // for if_stmt
    if node.kind == NodeKind::ND_IFSTMT {
        *depth += 1;
        read_node(node.if_node.as_ref().unwrap(), depth);
        if node.elsif_node.is_some() {
            read_node(node.elsif_node.as_ref().unwrap(), depth);
        }
        if node.else_node.is_some() {
            read_node(node.else_node.as_ref().unwrap(), depth);
        }
        *depth -= 1;
        return;
    }

    if node.kind == NodeKind::ND_IFCOND {
        *depth += 1;
        read_node(node.l.as_ref().unwrap(), depth);
        *depth -= 1;
        return;
    }

    if node.kind == NodeKind::ND_ELSE {
        *depth += 1;
        read_node(node.l.as_ref().unwrap(), depth);
        *depth -= 1;
        return;
    }

    if node.kind == NodeKind::ND_STMT {
        *depth += 1;
        read_node(node.l.as_ref().unwrap(), depth);
        *depth -= 1;
    }
    if node.kind == NodeKind::ND_STMT2 {
        *depth += 1;
        read_node(node.l.as_ref().unwrap(), depth);
        *depth -= 1;
        return;
    }

    if node.kind == NodeKind::ND_DECL {
        *depth += 1;
        read_node(node.l.as_ref().unwrap(), depth);
        *depth -= 1;
        return;
    }

    if node.kind == NodeKind::ND_FNCALL {
        return;
    }

    /*
        read binary_node.
    */
    *depth += 1;
    read_node(node.l.as_ref().unwrap(), depth);
    read_node(node.r.as_ref().unwrap(), depth);
    *depth -= 1;
}

fn print_node_info(node: &Node, depth: &mut usize) {
    print!("{}", " ".repeat(*depth * 2));
    match node.kind {
        NodeKind::ND_NUM => {
            println!("kind: {:?}, val: {}", node.kind, node.val);
        }
        NodeKind::ND_IDENT => {
            println!("kind: {:?}, str: {}", node.kind, node.str)
        }
        NodeKind::ND_FNCALL => {
            println!("kind: {:?}, fn_name: {}", node.kind, node.fn_name)
        }
        _ => {
            println!("kind: {:?}", node.kind);
        }
    }
}
