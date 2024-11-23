use std::collections::{HashMap, HashSet, LinkedList};

use crate::predicates::solvers;
use crate::predicates::values::{atom, term};

#[derive(Clone, Debug)]
struct TermInfo {
    term: term::Term,

    linked_terms: HashSet<String>,
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
                linked_terms: HashSet::new(),
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
        return self.terms.get(&name).unwrap().term.clone();
    }

    fn link_terms(&mut self, from: String, to: String) -> bool {
        if !self.terms.contains_key(&from) || !self.terms.contains_key(&to) {
            return false;
        }

        // we just checked, that there are some values.
        let (mut a, mut b) = (
            self.terms.remove(&from).unwrap(),
            self.terms.remove(&to).unwrap(),
        );

        match a.term.t() {
            term::TermType::Const => {
                match b.term.t() {
                    // this means, that consts are the same
                    term::TermType::Const => {}
                    term::TermType::Var => b.term.set_value(a.term.value().unwrap().to_string()),
                }
            }
            term::TermType::Var => match b.term.t() {
                term::TermType::Const => a.term.set_value(b.term.value().unwrap().to_string()),
                term::TermType::Var => {
                    let (a_name, b_name) = (a.term.value().unwrap(), b.term.value().unwrap());

                    let linked_terms: HashSet<String> = a
                        .linked_terms
                        .union(&b.linked_terms)
                        .map(|x| x.to_string())
                        .collect();

                    a.linked_terms = linked_terms
                        .clone()
                        .into_iter()
                        .filter(|x| *x != a_name)
                        .collect();
                    a.linked_terms.insert(b_name.clone());

                    b.linked_terms = linked_terms.into_iter().filter(|x| *x != b_name).collect();
                    b.linked_terms.insert(a_name);
                }
            },
        }

        println!("{:?} {:?}", a, b);

        self.terms.insert(from, a.clone());
        self.terms.insert(to, b.clone());

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
