use std::fmt;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
enum TermType {
    CONST,
    VAR,
}

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
pub struct Term {
    t: TermType,
    name: &'static str,
    value: &'static str,
}

impl Term {
    pub fn new_var(name: &'static str) -> Term {
        return Term {
            t: TermType::VAR,
            name: name,
            value: "",
        };
    }

    pub fn new_const(value: &'static str) -> Term {
        return Term {
            t: TermType::CONST,
            name: "",
            value: value,
        };
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn value(&self) -> &str {
        self.value
    }

    pub fn unify(self, _other: Term) -> bool {
        false
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.t {
            TermType::CONST => write!(f, "Const({})", self.name()),
            TermType::VAR => write!(f, "Var({}, {})", self.name(), self.value()),
        }
    }
}
