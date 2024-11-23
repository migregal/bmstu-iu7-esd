use std::fmt;

#[derive(Clone)]
pub struct Atom {
    name: &'static str,
    is_neg: bool,
    terms: Vec<String>,
}

impl Atom {
    pub fn new(name: &'static str, terms: Vec<&str>) -> Atom {
        return Atom {
            name: name,
            is_neg: false,
            terms: terms.iter().map(|x| x.to_string()).collect(),
        };
    }

    pub fn new_negative(name: &'static str, terms: Vec<&str>) -> Atom {
        return Atom {
            name: name,
            is_neg: true,
            terms: terms.iter().map(|x| x.to_string()).collect(),
        };
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn is_neg(&self) -> bool {
        self.is_neg
    }

    pub fn unify(&self, other: Atom) -> bool {
        if self.name() != other.name()
            || self.terms.len() != other.terms.len()
            || self.is_neg != other.is_neg
        {
            return false;
        }

        for term in self.terms.iter().zip(other.terms.iter()) {
            println!("{:?}", term)
            // println!("{:?}={}", term, term.0.unify(term.1))
        }

        false
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Atom({}: {:?})", self.name(), self.terms)
    }
}
