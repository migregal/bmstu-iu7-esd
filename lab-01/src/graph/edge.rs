use derive_more::Constructor;

use std::fmt;

use crate::graph::node;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Constructor)]
pub struct Edge {
    from: node::Node,
    to: node::Node,
    label: i32,
    used: bool,
}

impl Edge {
    pub fn from(&self) -> node::Node {
        return self.from;
    }

    pub fn to(&self) -> node::Node {
        return self.to;
    }

    pub fn label(&self) -> i32 {
        return self.label;
    }

    pub fn used(&self) -> bool {
        return self.used;
    }

    pub fn set_used(&mut self, used: bool) {
        self.used = used
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Edge({}): {} -> {}",
            self.label(),
            self.from.number(),
            self.to.number()
        )
    }
}
