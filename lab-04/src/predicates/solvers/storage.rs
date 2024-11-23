use std::collections::{HashMap, LinkedList};

use crate::predicates::values::{atom, term};

pub struct Storage {
    terms: HashMap<String, LinkedList<term::Term>>,

    atoms: HashMap<String, LinkedList<atom::Atom>>,
}

impl Storage {
    pub fn new() -> Storage {
        return Storage {
            terms: HashMap::new(),
            atoms: HashMap::new(),
        };
    }

    pub fn add_term(&mut self, term: term::Term) -> bool {
        let key = term.name().to_owned();

        if self.terms.get(&key).is_some() {
            return false;
        }

        self.terms.insert(key, LinkedList::new());
        true
    }

    pub fn add_atom(&mut self, atom: atom::Atom) {
        let key = atom.name().to_owned();

        match self.atoms.get_mut(&key) {
            Some(x) => x.push_back(atom.to_owned()),
            None => _ = self.atoms.insert(key, LinkedList::from([atom])),
        }
    }

    pub fn solve(&self, a: atom::Atom, b: atom::Atom) -> bool {
        return a.unify(b)
    }
}
