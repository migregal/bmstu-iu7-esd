use std::fmt;

use super::term::Term;

pub struct Atom {
    name: &'static str,
    is_neg: bool,
    terms: Vec<Term>,
}

impl Atom {
    pub fn new(name: &'static str, terms: Vec<Term>) -> Atom {
        return Atom {
            name: name,
            is_neg: false,
            terms: terms,
        };
    }

    pub fn new_negative(name: &'static str, terms: Vec<Term>) -> Atom {
        return Atom {
            name: name,
            is_neg: true,
            terms: terms,
        };
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn unify(&self, _other: Atom) -> bool {
        false
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Atom({})", self.name())
    }
}
