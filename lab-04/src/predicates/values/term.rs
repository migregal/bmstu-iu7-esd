use std::fmt;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
pub enum TermType {
    Const,
    Var,
}

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct Term {
    t: TermType,
    name: String,
    value: Option<String>,
}

impl Term {
    pub fn new_var(name: String) -> Term {
        return Term {
            t: TermType::Var,
            name: name,
            value: None,
        };
    }

    pub fn new_const(value: String) -> Term {
        return Term {
            t: TermType::Const,
            name: value.clone(),
            value: Some(value),
        };
    }

    pub fn t(&self) -> TermType {
        self.t
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn value(&self) -> Option<String> {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
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
            TermType::Var => {
                (this.value().is_none() && other.value().is_none())
                    || this.value().as_deref() != other.value().as_deref()
            }
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
                self.value().unwrap_or("None".to_string())
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
                self.value().unwrap_or("None".to_string())
            ),
        }
    }
}
