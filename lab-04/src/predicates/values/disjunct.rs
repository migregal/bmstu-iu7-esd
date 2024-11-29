use std::fmt;

use crate::predicates::solvers;

use crate::predicates::values::atom;

#[derive(Clone, Debug)]
pub struct Disjunct {
    name: &'static str,
    atoms: Vec<atom::Atom>,
}

impl Disjunct {
    pub fn new(name: &'static str, atoms: Vec<atom::Atom>) -> Disjunct {
        return Disjunct { name, atoms };
    }

    pub fn new_negative(name: &'static str, atoms: Vec<atom::Atom>) -> Disjunct {
        return Disjunct { name, atoms };
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn atoms(&self) -> Vec<atom::Atom> {
        self.atoms.clone()
    }
}

pub fn unify(storage: &mut dyn solvers::TermsStorage, this: Disjunct, other: Disjunct) -> bool {
    if this.name() != other.name() || this.atoms.len() != other.atoms.len() {
        return false;
    }

    let mut result = this.atoms();
    result.extend(other.atoms());

    for i in 0..result.len() {
        for j in i+1..result.len() {
            let (atom1, atom2) = (result[i].clone(), result[j].clone());

            if atom1.name() != atom2.name() {
                continue;
            }

            if atom1.is_neg() == atom2.is_neg() {
                if atom::unify(storage, atom1, atom2) {
                    result.remove(j);
                }

                continue;
            }

            if !atom::unify(storage, atom1, atom2) {
                continue;
            }

            result.remove(i);
            result.remove(j);
        }
    }


    true
}

impl fmt::Display for Disjunct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Disjunct({}: {:?})", self.name(), self.atoms)
    }
}
