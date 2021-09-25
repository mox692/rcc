use crate::tokenize::{Token, TokenKind, TokenReader};

pub struct Node {
    pub kind: NodeKind,
    pub l: Option<Box<Node>>,
    pub r: Option<Box<Node>>,
    pub val: i32,
}

pub enum NodeKind {
    ND_NUM,
    ND_ADD,
}

fn gen_num(tok: &mut TokenReader) -> Option<Box<Node>> {
    // num nodeが複数続くことは文法上ありえないので、そのまま返す.
    if tok.cur_tok().kind.eq(&TokenKind::NUM) {
        let node = Some(Box::new(Node {
            kind: NodeKind::ND_NUM,
            l: None,
            r: None,
            val: 0,
        }));
        tok.next();
        return node;
    }
    return None;
}

fn gen_add(tok: &mut TokenReader, l: Option<Box<Node>>, r: Option<Box<Node>>) -> Node {
    return Node {
        kind: NodeKind::ND_ADD,
        l: l,
        r: r,
        val: 0,
    };
}

// generate ND_ADD or ND_SUB node.
fn parse_add_sub(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut node = gen_num(tok);
    // process '+' token.
    loop {
        if tok.cur_tok().char.as_str() == "+" {
            // 第一引数には更新前のtokのcloneを渡して、第二引数に
            node = Some(Box::new(gen_add(
                &mut tok.clone(),
                node,
                gen_num(tok.next_tok()),
            )))
        } else {
            break;
        }
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

pub fn debug_nodes(node: &Node) {}
