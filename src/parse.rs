/*
    memo:
    * tokenの消費はgenXXX()系の関数の引数にtok.next_tok()を渡してtokenを進めるか、
    genXXX()系の関数内でtokenを進めるようにする.(parseXXX系の中では進めないようにする.)
*/

/*
    EBNF: ('&' express terminal symbol.)

    output = program
    program = stmt*
    stmt = ( assign | expr ) ";"
    assign = &ident ( "=" expr )*
    expr = ( add_sub | &ident )
    add_sub = mul_div( "+" mul_div | "-" mul_div )*
    mul_div = unary ( "*" unary | "/" unary )*
    unary = &num | &ident

    ·exprには即値と変数が混在しうる.有効な変数かどうかの判定は別途行う.
*/

use std::usize;

use crate::tokenize::{TokenKind, TokenReader};

#[derive(Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub l: Option<Box<Node>>,
    pub r: Option<Box<Node>>,
    // for num node. (should be 0 in other node.)
    pub val: i32,
    // for ident node. (should be "" in other node.)
    pub str: String,
}
#[derive(Clone)]
pub enum NodeKind {
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
}
impl NodeKind {
    fn to_string(&self) -> &str {
        match self {
            NodeKind::ND_NUM => "NUM",
            NodeKind::ND_ADD => "ADD",
            NodeKind::ND_SUB => "SUB",
            NodeKind::ND_MUL => "MUL",
            NodeKind::ND_DIV => "DIV",
            NodeKind::ND_EXPR => "EXPR",
            NodeKind::ND_IDENT => "IDENT",
            NodeKind::ND_ASSIGN => "ND_ASSIGN",
            NodeKind::ND_STMT => "ND_STMT",
            NodeKind::ND_RETURN => "ND_RETURN",
            _ => {
                panic!("Not impl NodeKind::to_string")
            }
        }
    }
}
impl PartialEq for NodeKind {
    // もっといい実装があるかも.
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(other.to_string())
    }
}
impl Eq for NodeKind {}

impl std::fmt::Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            NodeKind::ND_NUM => write!(f, "ND_NUM"),
            NodeKind::ND_ADD => write!(f, "ND_ADD"),
            NodeKind::ND_SUB => write!(f, "ND_SUB"),
            NodeKind::ND_MUL => write!(f, "ND_MUL"),
            NodeKind::ND_DIV => write!(f, "ND_DIV"),
            NodeKind::ND_EXPR => write!(f, "ND_EXPR"),
            NodeKind::ND_IDENT => write!(f, "ND_IDENT"),
            NodeKind::ND_ASSIGN => write!(f, "ND_ASSIGN"),
            NodeKind::ND_STMT => write!(f, "ND_STMT"),
            NodeKind::ND_RETURN => write!(f, "ND_RETURN"),

            _ => {
                panic!("Invalid Node Kind.")
            }
        }
    }
}

// unary_node = num
fn gen_unary_node(kind: NodeKind, tok: &mut TokenReader) -> Option<Box<Node>> {
    match kind {
        NodeKind::ND_NUM => return gen_num_node(tok),
        _ => panic!("Invalid node kind."),
    }
}

fn gen_expr(expr_node: Option<Box<Node>>, tok: &mut TokenReader) -> Option<Box<Node>> {
    let node = Some(Box::new(Node {
        kind: NodeKind::ND_EXPR,
        l: expr_node,
        r: None,
        val: 0,
        str: String::from(""),
    }));
    return node;
}

fn gen_num_node(tok: &mut TokenReader) -> Option<Box<Node>> {
    if tok.cur_tok().kind.eq(&TokenKind::NUM) {
        let node = Some(Box::new(Node {
            kind: NodeKind::ND_NUM,
            l: None,
            r: None,
            val: tok.cur_tok().value,
            str: String::from(""),
        }));
        tok.next();
        return node;
    }
    tok.error(String::from(format!(
        "expect num token, but got {}.",
        tok.cur_tok().kind
    )));
    panic!();
}

fn gen_stmt(tok: &mut TokenReader, node: Option<Box<Node>>) -> Option<Box<Node>> {
    let node = Some(Box::new(Node {
        kind: NodeKind::ND_STMT,
        l: node,
        r: None,
        val: 0,
        str: String::from(""),
    }));
    // MEMO: curを";"の次に指す
    tok.next();
    return node;
}

fn gen_ident_node(tok: &mut TokenReader) -> Option<Box<Node>> {
    let node = Some(Box::new(Node {
        kind: NodeKind::ND_IDENT,
        l: None,
        r: None,
        val: 0,
        str: String::from(tok.cur_tok().char),
    }));
    // MEMO: curを";"にして戻る.
    tok.next();
    return node;
}

fn gen_return_node(ret_stmt: Option<Box<Node>>) -> Option<Box<Node>> {
    let node = Some(Box::new(Node {
        kind: NodeKind::ND_RETURN,
        l: ret_stmt,
        r: None,
        val: 0,
        str: String::from(""),
    }));
    return node;
}

// num_node or ident_node.
fn gen_binary_node(kind: NodeKind, l: Option<Box<Node>>, r: Option<Box<Node>>) -> Node {
    return Node {
        kind: kind,
        l: l,
        r: r,
        val: 0,
        str: String::from(""),
    };
}

// nary = &num | &ident
fn parse_unary(tok: &mut TokenReader) -> Option<Box<Node>> {
    if tok.cur_tok().kind == TokenKind::NUM {
        return gen_num_node(tok);
    } else if tok.cur_tok().kind == TokenKind::IDENT {
        return gen_ident_node(tok);
    } else {
        panic!("Invalid unary!!!");
    }
}

// mul_div = unary ( "*" unary | "/" unary )*
fn parse_mul_div(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut node = parse_unary(tok);
    // let mut node = gen_unary_node(NodeKind::ND_NUM, tok);

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
    // process '+', '-' token.
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

// assing = ident ( "=" expr )*
fn parse_assign(tok: &mut TokenReader) -> Option<Box<Node>> {
    // 左辺のidentをparse.
    let mut node = gen_ident_node(tok);
    loop {
        if tok.expect("=") {
            // 右辺のexprをparse.
            node = Some(Box::new(gen_binary_node(
                NodeKind::ND_ASSIGN,
                node,
                parse_expr(tok.next_tok()),
            )));
        } else {
            break;
        }
    }
    return node;
}

// return ND_RETURN.
fn parse_return(tok: &mut TokenReader) -> Option<Box<Node>> {
    let node = gen_return_node(parse_expr(tok.next_tok()));
    return node;
}

// stmt = ( assign | "return" expr | expr ) ";"
fn parse_stmt(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut node: Option<Box<Node>>;
    if tok.cur_tok().kind == TokenKind::IDENT && tok.get_next_tok().char == "=" {
        // parse assign
        node = parse_assign(tok);
    } else if tok.cur_tok().kind == TokenKind::RETURN {
        // returnをparse.
        node = parse_return(tok);
    } else {
        // exprをparse.
        node = parse_expr(tok);
    }

    // MEMO: ここではcurは";"を指している.
    if tok.expect(";") {
        node = gen_stmt(tok, node);
        // MEMO: ここではcurは";"の次を指している.
        return node;
    }
    panic!("expect ';', but not found.")
}

// program = stmt*
fn parse_program(tok: &mut TokenReader) -> Vec<Box<Node>> {
    let mut nodes: Vec<Box<Node>> = Vec::new();
    loop {
        // TODO: ここ要る?
        if tok.cur_tok().char == "\0" {
            break;
        }
        let node = parse_stmt(tok);
        nodes.push(node.unwrap());
        if tok.cur_tok().char == "\0" {
            break;
        }
    }
    return nodes;
}

// generate several nodes, and return Vec<Node>.
// node = program
pub fn parse(tok: &mut TokenReader) -> Vec<Box<Node>> {
    // TODO: ini tok要る?
    consume_initial_tok(tok);
    let mut node: Vec<Box<Node>> = parse_program(tok);

    return node;
}

pub fn consume_initial_tok(tok: &mut TokenReader) {
    if tok.cur_tok().kind != TokenKind::INI {
        println!("expect INI TOKEN, but got {}", tok.cur_tok().kind);
        panic!("ERR");
    }
    tok.next();
}

pub fn debug_nodes(flag: bool, nodes: &Vec<Box<Node>>) {
    if !flag {
        return;
    }
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

    /*
        read unary_node.
    */
    // mainly for ND_NUM.
    if node.l.is_none() && node.r.is_none() {
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
            println!("kind: {}, val: {}", node.kind, node.val);
        }
        NodeKind::ND_IDENT => {
            println!("kind: {}, str: {}", node.kind, node.str)
        }
        _ => {
            println!("kind: {}", node.kind);
        }
    }
}
