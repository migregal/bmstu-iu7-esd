use std::fmt;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
enum TermType {
    CONST,
    VAR,
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
            t: TermType::VAR,
            name: name,
            value: None,
        };
    }

    pub fn new_const(value: &'static str) -> Term {
        return Term {
            t: TermType::CONST,
            name: "",
            value: Some(value),
        };
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn value(&self) -> Option<&str> {
        self.value
    }

    pub fn set_value(&mut self, value: &'static str) {
        match self.t {
            TermType::CONST => panic!("Can't assign value to const term"),
            TermType::VAR => self.value = Some(value),
        }
    }

    pub fn unify(&self, other: &Term) -> bool {
        if self.value().as_deref() != other.value().as_deref() {
            return false
        }

        true
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.t {
            TermType::CONST => write!(f, "Const({})", self.value().unwrap()),
            TermType::VAR => write!(f, "Var({}, {})", self.name(), self.value().unwrap_or("None")),
        }
    }
}

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.t {
            TermType::CONST => write!(f, "Const({})", self.value().unwrap()),
            TermType::VAR => write!(f, "Var({}, {})", self.name(), self.value().unwrap_or("None")),
        }
    }
}
