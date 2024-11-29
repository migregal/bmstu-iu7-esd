use std::fmt;

use crate::predicates::solvers;

use crate::predicates::values::term;

#[derive(Clone, Debug)]
pub struct Atom {
    name: String,
    is_neg: bool,
    terms: Vec<String>,
}

impl Atom {
    pub fn new(name: String, terms: Vec<String>) -> Atom {
        return Atom {
            name,
            is_neg: false,
            terms,
        };
    }

    pub fn new_negative(name: String, terms: Vec<String>) -> Atom {
        return Atom {
            name,
            is_neg: true,
            terms,
        };
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn is_neg(&self) -> bool {
        self.is_neg
    }

    pub fn terms(&self) -> Vec<String> {
        self.terms.clone()
    }
}

pub fn unify(storage: &mut dyn solvers::TermsStorage, this: Atom, other: Atom) -> bool {
    if this.name() != other.name()
        || this.terms.len() != other.terms.len()
        || this.is_neg != other.is_neg
    {
        return false;
    }

    let mut commands = Vec::<Box<dyn solvers::LinkTermsCommand>>::with_capacity(this.terms.len());

    for tuple in this
        .terms
        .iter()
        .map(|t| storage.get_term(t.to_string()))
        .zip(other.terms.iter().map(|t| storage.get_term(t.to_string())))
    {
        let t2 = tuple.clone();
        let unified = term::unify(tuple.0, tuple.1);
        // println!("{:?}={}", t2, unified);

        if !unified {
            return false;
        }

        commands.push(storage.get_link_cmd(t2.0.name(), t2.1.name()).unwrap());
    }

    storage.apply_cmds(commands);

    true
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Atom({}: {:?})", self.name(), self.terms)
    }
}
