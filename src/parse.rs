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
}
impl NodeKind {
    fn to_string(&self) -> &str {
        match self {
            &NodeKind::ND_ADD => "ADD",
            &NodeKind::ND_SUB => "SUB",
            &NodeKind::ND_NUM => "NUM",
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
            NodeKind::ND_ADD => write!(f, "ND_ADD"),
            NodeKind::ND_NUM => write!(f, "ND_NUM"),
            _ => {
                panic!("Invalid Node Kind.")
            }
        }
    }
}

fn gen_unary_node(kind: NodeKind, tok: &mut TokenReader) -> Option<Box<Node>> {
    match kind {
        NodeKind::ND_NUM => return gen_num(tok),
        _ => panic!("Invalid node kind."),
    }
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

fn gen_binary_node(kind: NodeKind, l: Option<Box<Node>>, r: Option<Box<Node>>) -> Node {
    return Node {
        kind: kind,
        l: l,
        r: r,
        val: 0,
    };
}

// generate ND_ADD or ND_SUB node.
fn parse_add_sub(tok: &mut TokenReader) -> Option<Box<Node>> {
    // はじめのtokenがnum nodeと決まりきってるので.
    let mut node = gen_unary_node(NodeKind::ND_NUM, tok);
    // process '+', '-' token.
    loop {
        match tok.cur_tok().char.as_str() {
            "+" => {
                node = Some(Box::new(gen_binary_node(
                    NodeKind::ND_ADD,
                    node,
                    gen_unary_node(NodeKind::ND_NUM, tok.next_tok()),
                )))
            }
            "-" => {
                node = Some(Box::new(gen_binary_node(
                    NodeKind::ND_SUB,
                    node,
                    gen_unary_node(NodeKind::ND_NUM, tok.next_tok()),
                )))
            }
            _ => break,
        };
    }

    return node;
}

// generate expression.
fn parse_expr(tok: &mut TokenReader) -> Option<Box<Node>> {
    let node = parse_add_sub(tok);
    return node;
}

// Get tokens, and convert to Node.
pub fn parse(tok: &mut TokenReader) -> Option<Box<Node>> {
    // TODO: ini tok要る?
    consume_initial_tok(tok);
    let node = parse_expr(tok);
    return node;
}

pub fn consume_initial_tok(tok: &mut TokenReader) {
    if tok.cur_tok().kind != TokenKind::INI {
        println!("expect INI TOKEN, but got {}", tok.cur_tok().kind);
        panic!("ERR");
    }
    tok.next();
}

pub fn debug_nodes(flag: bool, node: &Node) {
    if !flag {
        return;
    }
    println!("////////NODE DEBUG START////////");
    let mut depth = 0;
    read_node(node, &mut depth);
    println!("////////NODE DEBUG END////////");
}

pub fn read_node(node: &Node, depth: &mut usize) {
    print_node_info(node, depth);
    if node.l.is_none() && node.r.is_none() {
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
