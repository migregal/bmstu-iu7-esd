use std::collections::{HashMap, LinkedList};

use crate::predicates::solvers;
use crate::predicates::values::{atom, term};

#[derive(Clone, Debug)]
struct TermInfo {
    term: term::Term,

    linked_terms: LinkedList<String>,
}

pub struct Storage {
    terms: HashMap<String, TermInfo>,

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

        self.terms.insert(
            key,
            TermInfo {
                term: term,
                linked_terms: LinkedList::new(),
            },
        );
        true
    }

    pub fn add_atom(&mut self, atom: atom::Atom) {
        let key = atom.name().to_owned();

        match self.atoms.get_mut(&key) {
            Some(x) => x.push_back(atom.to_owned()),
            None => _ = self.atoms.insert(key, LinkedList::from([atom])),
        }
    }
}

impl solvers::TermsStorage for Storage {
    fn get_term(&self, name: String) -> term::Term {
        return self.terms.get(&name).unwrap().term;
    }

    fn link_terms(&mut self, from: String, to: String) -> bool {
        true
    }

    fn get_link_cmd(&self, from: String, to: String) -> Option<Box<dyn solvers::LinkTermsCommand>> {
        if !self.terms.contains_key(&from) || !self.terms.contains_key(&to) {
            return None;
        }

        return Some(Box::new(LinkTermsCommand { from: from, to: to }));
    }
}

struct LinkTermsCommand {
    from: String,
    to: String,
}

impl solvers::LinkTermsCommand for LinkTermsCommand {
    fn run(&self, mut storage: Box<dyn solvers::TermsStorage>) {
        storage.link_terms(self.from.clone(), self.to.clone());
    }
}
