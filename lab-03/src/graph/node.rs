use derive_more::Constructor;

use std::fmt;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Constructor, Debug)]
pub struct Node {
    n: i32,
}

impl Node {
    pub fn number(&self) -> i32 {
        self.n
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node({})", self.number())
    }
}
