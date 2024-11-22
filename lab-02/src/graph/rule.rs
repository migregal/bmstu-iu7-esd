use std::{collections::HashSet, fmt};

use super::node::Node;

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
enum RuleState {
    Open,
    Closed,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Rule {
    from: HashSet<Node>,
    to: Node,
    label: i32,
    state: RuleState,
}

impl Rule {
    pub fn new(label: i32, from: Vec<Node>, to: Node) -> Rule {
        return Rule {
            from: HashSet::from_iter(from),
            to,
            label,
            state: RuleState::Open,
        };
    }

    pub fn from(&self) -> Vec<Node> {
        return Vec::from_iter(self.from.iter().cloned());
    }

    pub fn to(&self) -> Node {
        return self.to;
    }

    pub fn label(&self) -> i32 {
        return self.label;
    }

    pub fn used(&self) -> bool {
        return self.state != RuleState::Open;
    }

    pub fn set_used(&mut self, used: bool) {
        self.state = if used {
            RuleState::Closed
        } else {
            RuleState::Open
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Rule({}): {:?} -> {}",
            self.label(),
            self.from(),
            self.to
        )
    }
}
