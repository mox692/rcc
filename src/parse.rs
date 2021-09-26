/*
    * tokenの消費はgenXXX()系の関数の引数にtok.next_tok()を渡してtokenを進めるか、
    genXXX()系の関数内でtokenを進めるようにする.(parseXXX系の中では進めないようにする.)
*/

use std::usize;

use crate::tokenize::{TokenKind, TokenReader};

#[derive(Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub l: Option<Box<Node>>,
    pub r: Option<Box<Node>>,
    pub val: i32,
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
            _ => {
                panic!("Invalid Node Kind.")
            }
        }
    }
}

// unary_node = num
fn gen_unary_node(kind: NodeKind, tok: &mut TokenReader) -> Option<Box<Node>> {
    match kind {
        NodeKind::ND_NUM => return gen_num(tok),
        _ => panic!("Invalid node kind."),
    }
}

fn gen_expr(expr_node: Option<Box<Node>>, tok: &mut TokenReader) -> Option<Box<Node>> {
    let node = Some(Box::new(Node {
        kind: NodeKind::ND_EXPR,
        l: expr_node,
        r: None,
        val: 0,
    }));
    return node;
}

fn gen_num(tok: &mut TokenReader) -> Option<Box<Node>> {
    // num nodeが複数続くことは文法上ありえないので、そのまま返す.
    if tok.cur_tok().kind.eq(&TokenKind::NUM) {
        let node = Some(Box::new(Node {
            kind: NodeKind::ND_NUM,
            l: None,
            r: None,
            val: tok.cur_tok().value,
        }));
        tok.next();
        return node;
    }
    return None;
}

fn gen_ident_node(tok: &mut TokenReader) -> Node {
    return Node {
        kind: NodeKind::ND_IDENT,
        l: None,
        r: None,
        val: 0,
    };
}

fn gen_binary_node(kind: NodeKind, l: Option<Box<Node>>, r: Option<Box<Node>>) -> Node {
    return Node {
        kind: kind,
        l: l,
        r: r,
        val: 0,
    };
}

// mul_div = unary ( "*" unary | "/" unary )*
fn parse_mul_div(tok: &mut TokenReader) -> Option<Box<Node>> {
    // はじめのtokenがnum nodeと決まりきってるので.
    let mut node = gen_unary_node(NodeKind::ND_NUM, tok);

    loop {
        match tok.cur_tok().char.as_str() {
            "*" => {
                node = Some(Box::new(gen_binary_node(
                    NodeKind::ND_MUL,
                    node,
                    gen_unary_node(NodeKind::ND_NUM, tok.next_tok()),
                )))
            }
            "/" => {
                node = Some(Box::new(gen_binary_node(
                    NodeKind::ND_DIV,
                    node,
                    gen_unary_node(NodeKind::ND_NUM, tok.next_tok()),
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
// expr = ( add_sub | ident ) ";"
fn parse_expr(tok: &mut TokenReader) -> Option<Box<Node>> {
    // ident
    if tok.cur_tok().kind == TokenKind::IDENT {
        // TODO: impl
        let node = Some(Box::new(gen_ident_node(tok)));
        return node;
    }

    // add_sub
    let mut node = parse_add_sub(tok);
    if tok.expect(";") {
        node = gen_expr(node, tok)
    } else {
        panic!("expect ';', but not found.")
    }
    tok.next();
    return node;
}

// assing = ident ( "=" expr )*
fn parse_assign(tok: &mut TokenReader) -> Option<Box<Node>> {
    // 左辺のidentをparse.
    let mut node = Some(Box::new(gen_ident_node(tok)));
    println!("identnode create.");
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

// stmt = assign | expr
fn parse_stmt(tok: &mut TokenReader) -> Option<Box<Node>> {
    let node: Option<Box<Node>>;
    // assignなstmtかどうかcheck.
    // 今の文法だと、この条件であればassignのはず.
    if tok.cur_tok().kind == TokenKind::IDENT && tok.get_next_tok().char == "=" {
        node = parse_assign(tok);
        return node;
    }

    // exprをparse.
    node = parse_expr(tok);
    return node;
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
    for node in nodes.iter() {
        // 現段階では1つのstatement毎にdepthをresetする.
        let mut depth = 0;
        read_node(node, &mut depth);
    }
    println!("////////NODE DEBUG END////////");
}

pub fn read_node(node: &Node, depth: &mut usize) {
    print_node_info(node, depth);
    // mainly for ND_NUM.
    if node.l.is_none() && node.r.is_none() {
        return;
    }

    // for ND_EXPR.
    if node.kind == NodeKind::ND_EXPR {
        *depth += 1;
        read_node(node.l.as_ref().unwrap(), depth);
        *depth -= 1;
        return;
    }

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
        _ => {
            println!("kind: {}", node.kind);
        }
    }
}
