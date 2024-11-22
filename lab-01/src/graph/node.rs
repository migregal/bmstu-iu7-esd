use derive_more::Constructor;

use std::fmt;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Constructor)]
pub struct Node {
    number: i32,
}

impl Node {
    pub fn number(&self) -> i32 {
        self.number
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node({})", self.number())
    }
}
