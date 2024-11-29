use std::fmt;

use std::collections::{HashMap, HashSet, LinkedList};

use crate::predicates::solvers;
use crate::predicates::values::{atom, term};

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

    pub fn print_atoms(&self, a: atom::Atom, b: atom::Atom) {
        println!("Atoms:");
        for i in vec![a, b].into_iter() {
            println!(
                "\t{}{}",
                match i.is_neg() {
                    true => "~",
                    false => "",
                },
                i.name()
            );

            for t in i.terms().into_iter().map(|x| self.terms.get(&x).unwrap()) {
                println!("\t\t{:?}", t)
            }
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

        // println!(
        //     "\n{} -> {}\n{:?} {:?}",
        //     from,
        //     to,
        //     self.terms.get(&from),
        //     self.terms.get(&to)
        // );

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
                    let linked_terms: HashSet<String> = a
                        .linked_terms
                        .union(&b.linked_terms)
                        .map(|x| x.to_string())
                        .collect();

                    a.linked_terms = linked_terms
                        .clone()
                        .into_iter()
                        .filter(|x| *x != from)
                        .collect();
                    a.linked_terms.insert(to.clone());

                    b.linked_terms = linked_terms.into_iter().filter(|x| *x != to).collect();
                    b.linked_terms.insert(from.clone());
                }
            },
        }

        // println!("{:?} {:?}", a, b);

        self.terms.insert(from, a.clone());
        self.terms.insert(to, b.clone());

        true
    }

    fn get_link_cmd(&self, from: String, to: String) -> Option<Box<dyn solvers::LinkTermsCommand>> {
        if !self.terms.contains_key(&from) || !self.terms.contains_key(&to) {
            return None;
        }

        return Some(Box::new(LinkTermsCommand { from, to }));
    }

    fn apply_cmds(&mut self, cmds: Vec<Box<dyn solvers::LinkTermsCommand>>) {
        cmds.into_iter().for_each(|c| c.run(self))
    }
}

struct LinkTermsCommand {
    from: String,
    to: String,
}

impl solvers::LinkTermsCommand for LinkTermsCommand {
    fn run(&self, storage: &mut dyn solvers::TermsStorage) {
        storage.link_terms(self.from.clone(), self.to.clone());
    }
}

#[derive(Clone)]
struct TermInfo {
    term: term::Term,

    linked_terms: HashSet<String>,
}

impl fmt::Display for TermInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.term.t() {
            term::TermType::Const => write!(f, "{}", self.term),
            term::TermType::Var => write!(f, "{} {:?}", self.term, self.linked_terms),
        }
    }
}

impl fmt::Debug for TermInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.term.t() {
            term::TermType::Const => write!(f, "{}", self.term),
            term::TermType::Var => write!(f, "{} {:?}", self.term, self.linked_terms),
        }
    }
}
