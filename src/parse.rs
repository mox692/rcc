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
        };
    }
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
            NodeKind::ND_EQ => "ND_EQ",
            NodeKind::ND_NEQ => "ND_NEQ",
            NodeKind::ND_BT => "ND_BT",
            NodeKind::ND_BE => "ND_BE",
            NodeKind::ND_LT => "ND_LT",
            NodeKind::ND_LE => "ND_LE",
            NodeKind::ND_IFSTMT => "ND_IFSTMT",
            NodeKind::ND_IF => "ND_IF",
            NodeKind::ND_ELSE => "ND_ELSE",
            NodeKind::ND_ELSIF => "ND_ELSIF",
            NodeKind::ND_IFCOND => "ND_IFCOND",
            NodeKind::ND_FOR => "ND_FOR",
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
            NodeKind::ND_EQ => write!(f, "ND_EQ"),
            NodeKind::ND_NEQ => write!(f, "ND_NEQ"),
            NodeKind::ND_BT => write!(f, "ND_BT"),
            NodeKind::ND_BE => write!(f, "ND_BE"),
            NodeKind::ND_LT => write!(f, "ND_LT"),
            NodeKind::ND_LE => write!(f, "ND_LE"),
            NodeKind::ND_IFSTMT => write!(f, "ND_IFSTMT"),
            NodeKind::ND_IF => write!(f, "ND_IF"),
            NodeKind::ND_ELSE => write!(f, "ND_ELSE"),
            NodeKind::ND_ELSIF => write!(f, "ND_ELSIF"),
            NodeKind::ND_IFCOND => write!(f, "ND_IFCOND"),
            NodeKind::ND_FOR => write!(f, "ND_FOR"),
            _ => {
                panic!("Invalid Node Kind.")
            }
        }
    }
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
        "expect num token, but got {}.",
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

// nary = &num | &ident
fn parse_unary(tok: &mut TokenReader) -> Option<Box<Node>> {
    if tok.cur_tok().kind == TokenKind::NUM {
        return gen_num_node(tok);
    } else if tok.cur_tok().kind == TokenKind::IDENT {
        return gen_ident_node(tok);
    } else {
        tok.error(String::from("unexpected unary."));
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
    loop {
        if tok.expect("=") {
            node = Some(Box::new(gen_binary_node(
                NodeKind::ND_ASSIGN,
                node,
                parse_equality(tok.next_tok()),
            )));
        } else {
            break;
        }
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
        node = gen_elsif_node(node, parse_stmts(tok.next_tok()));
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
        node = gen_if_node(node, parse_stmt(tok.next_tok()));
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
    let node = gen_else(parse_stmts(tok));
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

// forstmt = "for" "(" assign ";" equality ";" expr ")" stmts
fn parse_forstmt(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut node: Box<Node> = Box::new(Node {
        kind: NodeKind::ND_FOR,
        ..Default::default()
    });
    if tok.cur_tok().char == "(" {
        node.for_node_first_assign = parse_assign(tok.next_tok());
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
        node.for_node_stmts = parse_stmts(tok.next_tok());
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

// stmt = ( assign | return | equality ) ";"
fn parse_stmt(tok: &mut TokenReader) -> Option<Box<Node>> {
    let mut node: Option<Box<Node>>;
    if tok.cur_tok().kind == TokenKind::IDENT && tok.get_next_tok().char == "=" {
        node = parse_assign(tok);
    } else if tok.cur_tok().kind == TokenKind::RETURN {
        node = parse_return(tok);
    } else {
        node = parse_equality(tok);
    }

    // MEMO: ここではcurは";"を指している.
    if tok.expect(";") {
        node = gen_stmt(tok, node);
        // MEMO: ここではcurは";"の次を指している.
        return node;
    }

    tok.error(String::from("expect ';', but not found."));
    panic!("");
}

// stmts = ( stmt | ifstmt | forstmt )
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
    node = parse_stmt(tok);
    return node;
}

// program = stmts*
fn parse_program(tok: &mut TokenReader) -> Vec<Box<Node>> {
    let mut nodes: Vec<Box<Node>> = Vec::new();
    loop {
        // TODO: ここ要る?
        if tok.cur_tok().char == "\0" {
            break;
        }
        let node = parse_stmts(tok);
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
