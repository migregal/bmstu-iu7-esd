use crate::predicates::values::atom;

use super::storage;

pub struct Solver {
    storage: storage::Storage,
}

impl Solver {
    pub fn new(storage: storage::Storage) -> Solver {
        return Solver {
            storage: storage,
        };
    }

    pub fn solve(&mut self, a: atom::Atom, b: atom::Atom) -> bool {
        self.storage.print_atoms(a.clone(), b.clone());

        let res = atom::unify(&mut self.storage, a.clone(), b.clone());

        self.storage.print_atoms(a, b);

        res
    }
}
