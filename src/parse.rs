use crate::tokenize::Token;

pub struct Node {
    pub kind: NodeKind,
}
impl Node {
    fn new_node() {}
}

pub enum NodeKind {
    Node,
}

pub fn parse(token: Vec<Token>) -> Node {
    return Node {
        kind: NodeKind::Node,
    };
}
