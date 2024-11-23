use crate::predicates::values::atom;

use super::storage;

pub struct Solver {
    storage: storage::Storage,
}

impl Solver {
    pub fn new(storage: storage::Storage) -> Solver {
        return Solver { storage };
    }

    pub fn solve(&self, a: atom::Atom, b: atom::Atom) -> bool {
        return a.unify(b);
    }
}
