use std::fmt;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
pub enum TermType {
    Const,
    Var,
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct Term {
    t: TermType,
    name: &'static str,
    value: Option<&'static str>,
}

impl Term {
    pub fn new_var(name: &'static str) -> Term {
        return Term {
            t: TermType::Var,
            name: name,
            value: None,
        };
    }

    pub fn new_const(value: &'static str) -> Term {
        return Term {
            t: TermType::Const,
            name: value,
            value: Some(value),
        };
    }

    pub fn t(&self) -> TermType {
        self.t
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn value(&self) -> Option<&str> {
        self.value
    }

    pub fn set_value(&mut self, value: &'static str) {
        match self.t {
            TermType::Const => panic!("Can't assign value to const term"),
            TermType::Var => self.value = Some(value),
        }
    }
}

pub fn unify(this: Term, other: Term) -> bool {
    match this.t {
        TermType::Const => match other.t {
            TermType::Const => this.value().as_deref() != other.value().as_deref(),
            TermType::Var => other
                .value()
                .is_none_or(|x| this.value().is_some_and(|y| x == y)),
        },
        TermType::Var => match other.t {
            TermType::Const => this
                .value()
                .is_none_or(|x| this.value().is_some_and(|y| x == y)),
            TermType::Var => (this.value().is_none() && other.value().is_none())
                || this.value().as_deref() != other.value().as_deref(),
        },
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.t {
            TermType::Const => write!(f, "Const({})", self.value().unwrap()),
            TermType::Var => write!(
                f,
                "Var({}, {})",
                self.name(),
                self.value().unwrap_or("None")
            ),
        }
    }
}

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.t {
            TermType::Const => write!(f, "Const({})", self.value().unwrap()),
            TermType::Var => write!(
                f,
                "Var({}, {})",
                self.name(),
                self.value().unwrap_or("None")
            ),
        }
    }
}
