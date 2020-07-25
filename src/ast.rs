use id_arena::{Id, Arena};
use std::collections::HashMap;

pub type StringId = Id<String>;

#[derive(Default)]
pub struct Context {
    idents: Arena<String>,
    already_interned: HashMap<String,StringId>,
    nodes: Arena<Node>,
}

impl Context {
    pub fn intern<S: AsRef<str> + Into<String>>(&mut self, s: S) -> StringId {
        if let Some(id) = self.already_interned.get(s.as_ref()) {
            return *id;
        }
        let s = s.into();
        let id = self.idents.alloc(s.clone());
        self.already_interned.insert(s,id);

        id
    }

    pub fn new_node(&mut self, node: Node) -> NodeId {
        self.nodes.alloc(node)
    }

    pub fn new_identifier<S: AsRef<str> + Into<String>>(&mut self, s: S) -> NodeId {
        let s = self.intern(s);
        self.nodes.alloc(Node::Identifier(s))
    }

    pub fn node_ref(&self, id: NodeId) -> &Node {
        &self.nodes[id]
    }

    pub fn interned(&self, id: StringId) -> &str {
        &self.idents[id]
    }
}

pub type NodeId = Id<Node>;

pub enum Node {
    Identifier(StringId),
    Addition(NodeId,NodeId),
    Subtraction(NodeId,NodeId),
    Multiplication(NodeId,NodeId),
    Division(NodeId,NodeId),
    LeftShift(NodeId,NodeId),
    RightShift(NodeId,NodeId),
    Const(i64),
    Negation(NodeId),
    Conditional(NodeId,NodeId,NodeId),
}
