use crate::tokenize::Token;

pub struct Node {
    pub kind: NodeKind,
}

pub enum NodeKind {
    Node,
}

pub fn parse(token: Vec<Token>) -> Node {
    return Node {
        kind: NodeKind::Node,
    };
}
